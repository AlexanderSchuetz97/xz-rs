extern crate std;

use crate::{XzDecoder, XzError, XzNextBlockResult};
use alloc::boxed::Box;
use alloc::vec;
use alloc::vec::Vec;
use core::num::NonZeroUsize;
use std::io::Read;

impl std::error::Error for XzError {}
#[derive(Debug)]
pub struct XzReader<R: Read + 'static> {
    /// the inner decoder, on the heap.
    decoder: Box<XzDecoder<'static>>,
    /// the underlying stream
    reader: R,
    /// A buffer
    buffer: Vec<u8>,
    /// Amount of bytes in the buffer we have consumed.
    buffer_consumed: usize,
    /// Amount of bytes in the buffer available for consumption.
    buffer_fill_count: usize,
    /// Are we at the end of a valid xz stream and should return eof?
    eos: bool,
}

impl<R: Read> XzReader<R> {
    /// Creates a new instance of `XzReader`
    #[allow(clippy::missing_panics_doc)] //We never actually panic.
    pub fn new(r: R) -> Self {
        Self::new_with_buffer_size(r, NonZeroUsize::new(8192).expect("Impossible to fail"))
    }
    pub fn new_with_buffer_size(r: R, buffer_size: NonZeroUsize) -> Self {
        Self::new_with_buffer_size_and_decoder(r, buffer_size, XzDecoder::in_heap())
    }

    pub fn new_with_buffer_size_and_decoder(
        r: R,
        buffer_size: NonZeroUsize,
        decoder: Box<XzDecoder<'static>>,
    ) -> Self {
        Self {
            decoder,
            reader: r,
            buffer: vec![0; buffer_size.into()],
            buffer_consumed: 0,
            buffer_fill_count: 0,
            eos: false,
        }
    }

    pub fn new_with_existing_buffered_data(
        r: R,
        buffer_size: NonZeroUsize,
        initial_data_in_buffer: impl AsRef<[u8]>,
    ) -> Self {
        Self::new_with_existing_buffered_data_and_decoder(
            r,
            buffer_size,
            initial_data_in_buffer,
            XzDecoder::in_heap(),
        )
    }

    pub fn new_with_existing_buffered_data_and_decoder(
        r: R,
        buffer_size: NonZeroUsize,
        initial_data_in_buffer: impl AsRef<[u8]>,
        decoder: Box<XzDecoder<'static>>,
    ) -> Self {
        let initial = initial_data_in_buffer.as_ref();
        let mut reader = Self::new_with_buffer_size_and_decoder(
            r,
            buffer_size.max(NonZeroUsize::new(initial.len()).unwrap_or(NonZeroUsize::MIN)),
            decoder,
        );
        reader.buffer.as_mut_slice()[0..initial.len()].copy_from_slice(initial);
        reader.buffer_fill_count = initial.len();
        reader
    }

    /// Reset the decoder to possibly decode the next fresh stream.
    pub fn reset(&mut self) {
        self.eos = false;
        self.decoder.reset();
    }

    /// Returns true if the xz stream is end of a valid xz stream.
    pub const fn is_eos(&self) -> bool {
        self.eos
    }

    /// Ensure that the buffer has at least 1 more readable byte. Otherwise, fill the inner buffer.
    fn fill_buffer(&mut self) -> std::io::Result<()> {
        debug_assert!(self.buffer_fill_count >= self.buffer_consumed);

        if self.buffer_consumed == self.buffer_fill_count {
            self.buffer_fill_count = self.reader.read(&mut self.buffer)?;
            if self.buffer_fill_count == 0 {
                return Err(std::io::Error::from(std::io::ErrorKind::UnexpectedEof));
            }
            self.buffer_consumed = 0;
        }

        Ok(())
    }

    /// Take a peek at raw data without consuming it.
    /// The param fn is guaranteed to be called with at least 1 byte of data.
    /// # Errors
    /// propagated from the underlying stream.
    /// This fn fails with `UnexpectedEof` if no data is available and no data can be read from the stream.
    pub fn peek_inner<T>(&mut self, peeker: impl FnOnce(&[u8]) -> T) -> std::io::Result<T> {
        self.fill_buffer()?;
        Ok(peeker(
            &self.buffer[self.buffer_consumed..self.buffer_fill_count],
        ))
    }

    /// Read raw bytes, bypassing the decoder.
    pub fn read_inner<T>(&mut self, reader: impl FnOnce(&mut dyn Read) -> T) -> T {
        struct ReadInner<'a, R: Read + 'static>(&'a mut XzReader<R>);

        impl<R: Read> Read for ReadInner<'_, R> {
            fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
                debug_assert!(self.0.buffer_fill_count >= self.0.buffer_consumed);
                if self.0.buffer_fill_count != self.0.buffer_consumed {
                    let available = self.0.buffer_fill_count - self.0.buffer_consumed;
                    let to_copy = std::cmp::min(available, buf.len());
                    let source = &self.0.buffer[self.0.buffer_consumed..][..to_copy];
                    buf[..to_copy].copy_from_slice(source);
                    self.0.buffer_consumed += to_copy;
                    return Ok(to_copy);
                }

                self.0.reader.read(buf)
            }
        }

        reader(&mut ReadInner(self))
    }

    /// Returns the underlying reader as well as the (possibly empty)
    /// buffer that may contain some unprocessed data.
    pub fn into_inner(mut self) -> (R, Vec<u8>) {
        debug_assert!(self.buffer_fill_count >= self.buffer_consumed);
        if self.buffer_consumed != 0 {
            self.buffer
                .copy_within(self.buffer_consumed..self.buffer_fill_count, 0);
            self.buffer_fill_count -= self.buffer_consumed;
        }

        self.buffer.truncate(self.buffer_fill_count);
        (self.reader, self.buffer)
    }
}
impl<R: Read> Read for XzReader<R> {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        if buf.is_empty() {
            return Ok(0);
        }

        if self.eos {
            return Ok(0);
        }

        loop {
            debug_assert!(self.buffer_fill_count >= self.buffer_consumed);
            self.fill_buffer()?;

            return match self.decoder.decode(
                &self.buffer.as_slice()[self.buffer_consumed..self.buffer_fill_count],
                buf,
            ) {
                Ok(XzNextBlockResult::NeedMoreData(in_count, outcount)) => {
                    self.buffer_consumed += in_count;
                    if outcount == 0 {
                        continue;
                    }
                    Ok(outcount)
                }
                Ok(XzNextBlockResult::EndOfStream(_, outcount)) => {
                    self.eos = true;
                    Ok(outcount)
                }
                Err(err) => Err(std::io::Error::new(std::io::ErrorKind::InvalidData, err)),
            };
        }
    }
}
