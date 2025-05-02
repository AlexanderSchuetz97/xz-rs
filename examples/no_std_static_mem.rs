use spin::mutex::SpinMutex;
use xz4rust::{XzNextBlockResult, XzStaticDecoder};

// DICT_SIZE_PROFILE_0 is the size of the dictionary in bytes! It has a direct impact on the size of the variable.
// In this configuration this entire variable is about 300k in size, which will be placed in your binary.
// If you are willing to use unsafe code then you should also be able to place it in zeroed memory. (if the linker permits)
// In this example we use no unsafe code and just accept that the binary gets bigger.
static DECODER: SpinMutex<XzStaticDecoder<{ xz4rust::DICT_SIZE_PROFILE_0 }>> =
    SpinMutex::new(XzStaticDecoder::new());
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
