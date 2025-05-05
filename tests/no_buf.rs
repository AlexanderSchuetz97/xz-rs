use xz4rust::{XzDecoder, XzError};

#[test]
pub fn test_no_buf() {
    let input = include_bytes!("../test_files/java_native_utils_riscv64.so.xz");
    let exp = include_bytes!("../test_files/java_native_utils_riscv64.so");

    let mut out_vec = Vec::new();
    out_vec.resize(exp.len() - 1, 0);
    let mut s2 = out_vec.as_mut_slice();

    let mut n = XzDecoder::default();
    let mut r = input.as_slice();
    loop {
        let s2l = s2.len();
        match n.decode(&r[..1], &mut s2[..1.min(s2l)]) {
            Ok(n) => {
                if n.is_end_of_stream() {
                    panic!("end of stream");
                }

                r = &r[n.input_consumed()..];
                s2 = &mut s2[n.output_produced()..];
            }
            Err(err) => {
                assert_eq!(XzError::NeedsLargerInputBuffer, err);
                break;
            }
        }
    }

    assert_eq!(out_vec.as_slice(), &exp[..exp.len() - 1]);
}
