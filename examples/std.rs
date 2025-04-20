use std::fs::File;
use std::io::Read;
use xz_rs::XzReader;

fn main() -> std::io::Result<()> {
    let file = File::open("test_files/good-1-block_header-1.xz")?;
    let mut reader = XzReader::new(file);

    let mut result = Vec::new();
    reader.read_to_end(&mut result)?;

    println!("{}", String::from_utf8_lossy(&result));
    Ok(())
}
