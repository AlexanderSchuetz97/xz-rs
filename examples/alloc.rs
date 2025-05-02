use xz4rust::{XzDecoder, XzNextBlockResult};

fn main() {
    //This file contains Hello\nWorld!
    let compressed_data = include_bytes!("../test_files/good-1-block_header-1.xz");
    let mut decompressed_data = Vec::new();

    let initial_alloc_size = xz4rust::DICT_SIZE_MIN;
    //Note: This is 3GB, decide yourself if you want the decoder to allocate this much memory if the possibly untrustworthy input file requires it.
    let max_alloc_size = xz4rust::DICT_SIZE_MAX;
    let mut decoder = XzDecoder::in_heap_with_alloc_dict_size(initial_alloc_size, max_alloc_size);

    let mut input_position = 0usize;
    loop {
        let mut temp_buffer = [0u8; 4096];
        match decoder.decode(&compressed_data[input_position..], &mut temp_buffer) {
            Ok(XzNextBlockResult::NeedMoreData(input_consumed, output_produced)) => {
                input_position += input_consumed;
                decompressed_data.extend_from_slice(&temp_buffer[..output_produced]);
            }
            Ok(XzNextBlockResult::EndOfStream(_, output_produced)) => {
                decompressed_data.extend_from_slice(&temp_buffer[..output_produced]);
                break;
            }
            Err(err) => panic!("Decompression failed {}", err),
        };
    }

    //This obviously requires std, but is just for illustrative purposes, you can do something else with the data...
    println!(
        "Decompressed contents: {}",
        String::from_utf8_lossy(&decompressed_data)
    );
    println!("Finished!");
}
