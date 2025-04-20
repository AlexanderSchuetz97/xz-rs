use xz4rust::XzDecoder;

#[test]
pub fn test_min() {
    let input = include_bytes!("../test_files/java_native_utils_riscv64.so.xz");
    let exp = include_bytes!("../test_files/java_native_utils_riscv64.so");

    let mut out_vec = Vec::new();

    let mut n = XzDecoder::default();
    let mut buf = [0u8; 1];
    let mut r = input.as_slice();
    loop {
        let n = n.decode(&r[..1], &mut buf).unwrap();
        r = &r[n.input_consumed()..];
        let x = &buf.as_slice()[0..n.output_produced()];
        out_vec.extend_from_slice(x);
        if n.is_end_of_stream() {
            break;
        }
    }

    assert_eq!(out_vec.as_slice(), exp);
}
