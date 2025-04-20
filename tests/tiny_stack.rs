use std::io::Read;
use std::thread;
use xz4rust::XzReader;

#[test]
fn test_tiny_stack() {
    let r = thread::Builder::new()
        .stack_size(8096)
        .spawn(move || {
            let data = include_bytes!("../test_files/good-1-block_header-1.xz");
            let mut result = Vec::new();
            let mut decoder = XzReader::new(data.as_slice());
            decoder.read_to_end(&mut result).unwrap();
            println!("{}", String::from_utf8_lossy(&result));
        })
        .unwrap();
    r.join().unwrap();
}
