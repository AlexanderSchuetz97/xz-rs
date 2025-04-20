use spin::mutex::SpinMutex;
use xz4rust::{XzNextBlockResult, XzStaticDecoder};

// 65536 is the size of the dictionary!
// This entire variable is about 100k in size, which will be placed in your binary.
// If you are willing to use unsafe code then you can also use zeroed memory.
// This will work as long as you reset the decoder before using it.
// In this case we use no unsafe code and just accept that the binary gets bigger.
static DECODER: SpinMutex<XzStaticDecoder<65536>> = SpinMutex::new(XzStaticDecoder::new());
fn main() {
    //This file contains Hello\nWorld!
    let compressed_data = include_bytes!("../test_files/good-1-block_header-1.xz");

    let mut decompressed_data_buffer = [0u8; 16];

    let mut decoder = DECODER.lock();
    let mut input_position = 0usize;
    loop {
        match decoder.decode(
            &compressed_data[input_position..],
            &mut decompressed_data_buffer,
        ) {
            Ok(XzNextBlockResult::NeedMoreData(input_consumed, output_produced)) => {
                input_position += input_consumed;
                if output_produced > 0 {
                    // Note: We know this input file contains only ascii characters
                    // and no multibyte which might be split at the edge of a buffer!
                    print!(
                        "{}",
                        std::str::from_utf8(&decompressed_data_buffer[..output_produced]).unwrap()
                    );
                }
            }
            Ok(XzNextBlockResult::EndOfStream(_, output_produced)) => {
                if output_produced > 0 {
                    // Note: We know this input file contains only ascii characters
                    // and no multibyte which might be split at the edge of a buffer!
                    print!(
                        "{}",
                        std::str::from_utf8(&decompressed_data_buffer[..output_produced]).unwrap()
                    );
                }
                println!();
                println!("Finished!");
                break;
            }
            Err(err) => panic!("Decompression failed {}", err),
        };
    }
}
