use xz_rs::XzDecoder;

#[test]
fn test_decoder_is_greedy() {
    let input = include_bytes!("../test_files/java_native_utils_riscv64.so.xz");
    let expected = include_bytes!("../test_files/java_native_utils_riscv64.so");
    let mut buf = vec![0u8; expected.len() + 128];
    let mut decoder = XzDecoder::default();
    let res = decoder
        .decode(input.as_slice(), buf.as_mut_slice())
        .unwrap();
    assert_eq!(res.output_produced(), expected.len());
    assert_eq!(&buf[..expected.len()], expected.as_slice());
    assert_eq!(res.is_end_of_stream(), true);
    assert_eq!(res.input_consumed(), input.len());
}
