use std::io::Read;
use xz_rs::XzReader;

#[test]
fn run_perf() {
    let env_var = std::env::var("TEST_PERF").unwrap_or("false".to_string());
    if "true" != env_var.as_str() {
        return;
    }

    let data = include_bytes!("../test_files/java_native_utils_amd64.so2.xz");

    let mut m = Vec::new();
    let mut n = XzReader::new(data.as_slice());
    let esize = n.read_to_end(&mut m).unwrap();
    assert_eq!(esize, 844584);

    for _ in 0..0x1_00 {
        let mut n = XzReader::new(data.as_slice());
        n.read_exact(&mut m).unwrap();
        std::hint::black_box(&mut m);
    }
}
