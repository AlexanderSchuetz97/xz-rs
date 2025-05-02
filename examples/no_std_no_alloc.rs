use xz4rust::{XzDecoder, XzNextBlockResult};

/// I am aware that the print! macro is not available in no_std, but other than that everything
/// here should work in no_std environments.
fn main() {
    //This file contains Hello\nWorld!
    let compressed_data = include_bytes!("../test_files/good-1-block_header-1.xz");
    // The size of the dictionary depends on the input file, this one has a 64kib dictionary.
    // If you use xz-utils, you can set the dict size when encoding.
    // The largest preset in xz-utils (-9) uses a 65mb dictionary.
    // Using a dictionary that is too small will cause an err when decoding.
    // Note: You may have to move this off the stack to the heap or static memory if this becomes too large for your stack.
    let mut dictionary_buffer = [0u8; 65536];
    let mut decompressed_data_buffer = [0u8; 16];

    let mut decoder = XzDecoder::with_fixed_size_dict(&mut dictionary_buffer);
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
