use std::fs::File;
use std::io::Read;
use std::{env, io};
use xz_rs::XzReader;

fn do_io(path: &str) -> io::Result<()> {
    let mut reader = XzReader::new(File::open(path)?);
    let mut buffer = Vec::with_capacity(0x10000);
    buffer.resize(0x10000, 0);
    loop {
        if reader.read(&mut buffer)? == 0 {
            return Ok(());
        }
    }
}

fn main() {
    let args = env::args().collect::<Vec<String>>();
    if args.len() == 1 {
        eprintln!(
            "xzcheck verifies that .xz files are valid and can be decoded using the xz-rs library."
        );
        eprintln!("xzcheck exits with code 0 on success or 255 on failure. A error message will be printed to stderr.");
        eprintln!("Usage: xzcheck <file>");
        std::process::exit(0);
    }
    if args.len() != 2 {
        eprintln!("Usage: xzcheck <file>");
        std::process::exit(1);
    }

    if let Err(err) = do_io(&args[1]) {
        eprintln!("Error: {}", err);
        std::process::exit(1);
    }

    std::process::exit(0);
}
