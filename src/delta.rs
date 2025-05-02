use crate::decoder::{DecodeResult, XzDictBuffer, XzInOutBuffer};
use crate::XzError;
use core::num::NonZeroUsize;

/// Delta filter decoder
#[derive(Debug)]
pub struct DeltaDecoder {
    /// index in the history buffer.
    index: usize,
    /// Current distance
    distance: usize,
    /// history buffer, initially 0, contains the last output byte.
    history: [u8; 256],
}

impl Default for DeltaDecoder {
    fn default() -> Self {
        Self::new()
    }
}

impl DeltaDecoder {
    /// Constructor
    pub const fn new() -> Self {
        Self {
            index: 0,
            distance: 0,
            history: [0; 256],
        }
    }

    /// Reset the delta decoder to use the given distance.
    pub const fn reset(&mut self, distance: NonZeroUsize) {
        debug_assert!(distance.get() <= 256);
        self.index = 0;
        self.distance = distance.get();
        self.history = [0; 256];
    }

    /// Run the delta filter on fewer or equal than distance bytes of output.
    fn decode_small_buffer(&mut self, produced_data: &mut [u8]) {
        debug_assert!(produced_data.len() <= self.distance);
        if produced_data.len() < self.distance - self.index {
            if produced_data.is_empty() {
                return;
            }

            for (i, hi) in (self.index..self.index + produced_data.len()).enumerate() {
                produced_data[i] = self.history[hi].wrapping_add(produced_data[i]);
                self.history[hi] = produced_data[i];
            }
            self.index += produced_data.len();
            return;
        }

        let mut i = 0;
        for hi in self.index..self.distance {
            produced_data[i] = self.history[hi].wrapping_add(produced_data[i]);
            self.history[hi] = produced_data[i];
            i += 1;
        }

        self.index = produced_data.len() - i;
        for hi in 0..self.index {
            produced_data[i] = self.history[hi].wrapping_add(produced_data[i]);
            self.history[hi] = produced_data[i];
            i += 1;
        }
    }

    /// Run the delta filter on the `produced_data`.
    fn decode(&mut self, produced_data: &mut [u8]) {
        debug_assert!(self.distance != 0);
        if produced_data.len() <= self.distance {
            self.decode_small_buffer(produced_data);
            return;
        }

        let mut i = 0;
        for hi in self.index..self.distance {
            produced_data[i] = self.history[hi].wrapping_add(produced_data[i]);
            i += 1;
        }

        for hi in 0..self.index {
            produced_data[i] = self.history[hi].wrapping_add(produced_data[i]);
            i += 1;
        }

        for i in self.distance..produced_data.len() {
            produced_data[i] = produced_data[i - self.distance].wrapping_add(produced_data[i]);
        }

        self.index = 0;
        self.history.as_mut_slice()[..self.distance]
            .copy_from_slice(&produced_data[produced_data.len() - self.distance..]);
    }

    /// Run the lzma decoder and then process the result using the filter.
    pub fn run<T: FnMut(&mut XzInOutBuffer, &mut XzDictBuffer) -> Result<DecodeResult, XzError>>(
        &mut self,
        mut next_filter: T,
        b: &mut XzInOutBuffer,
        d: &mut XzDictBuffer,
    ) -> Result<DecodeResult, XzError> {
        let base_pos = b.output_position();
        let ret = next_filter(b, d)?;
        let produced_data = b.output_slice_look_back_mut(base_pos);
        self.decode(produced_data);
        Ok(ret)
    }
}
