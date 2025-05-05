use std::io::Read;
use std::num::NonZeroUsize;
use xz4rust::{XzDecoder, XzError, XzReader};

#[test]
pub fn dict2big() {
    let input = include_bytes!("../test_files/java_native_utils_riscv64.so.xz");


    let mut r = XzReader::new_with_buffer_size_and_decoder(input.as_slice(), NonZeroUsize::new(4096).unwrap(), XzDecoder::in_heap_with_alloc_dict_size(4096, 4096));
    let err = r.read_to_end(&mut Vec::new()).unwrap_err();
    let n : XzError = err.downcast().unwrap();

    assert_eq!(XzError::DictionaryTooLarge(8388608), n);
}
