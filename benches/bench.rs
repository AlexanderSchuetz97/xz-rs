#![feature(test)]

extern crate test;

use std::io::Read;
use test::{black_box, Bencher};
use xz_rs::XzReader;

#[bench]
fn b1(b: &mut Bencher) {
    let data = include_bytes!("../test_files/good-1-block_header-1.xz");

    let mut m = Vec::new();
    let mut n = XzReader::new(data.as_slice());
    let esize = n.read_to_end(&mut m).unwrap();
    assert_eq!(esize, 13);

    b.iter(|| {
        let mut n = XzReader::new(data.as_slice());
        n.read_exact(m.as_mut_slice()).unwrap();
        black_box(&mut m);
    });
}

#[bench]
fn b1_native(b: &mut Bencher) {
    let data = include_bytes!("../test_files/good-1-block_header-1.xz");

    let mut m = Vec::new();
    let mut n = XzReader::new(data.as_slice());
    let esize = n.read_to_end(&mut m).unwrap();
    assert_eq!(esize, 13);

    b.iter(|| {
        let mut n = xz2::read::XzDecoder::new(data.as_slice());
        n.read_exact(&mut m).unwrap();
        black_box(&mut m);
    });
}

#[bench]
fn b2_this(b: &mut Bencher) {
    let data = include_bytes!("../test_files/java_native_utils_amd64.so2.xz");

    let mut m = Vec::new();
    let mut n = XzReader::new(data.as_slice());
    let esize = n.read_to_end(&mut m).unwrap();
    assert_eq!(esize, 844584);

    b.iter(|| {
        let mut n = XzReader::new(data.as_slice());
        n.read_exact(m.as_mut_slice()).unwrap();
        black_box(&mut m);
    });
}

#[bench]
fn b2_native(b: &mut Bencher) {
    let data = include_bytes!("../test_files/java_native_utils_amd64.so2.xz");

    let mut m = Vec::new();
    let mut n = XzReader::new(data.as_slice());
    let esize = n.read_to_end(&mut m).unwrap();
    assert_eq!(esize, 844584);

    b.iter(|| {
        let mut n = xz2::read::XzDecoder::new(data.as_slice());
        n.read_exact(&mut m).unwrap();
        black_box(&mut m);
    });
}

#[bench]
fn b2_emb(b: &mut Bencher) {
    let data = include_bytes!("../test_files/java_native_utils_amd64.so2.xz");

    let mut m = Vec::new();
    let mut n = XzReader::new(data.as_slice());
    let esize = n.read_to_end(&mut m).unwrap();
    assert_eq!(esize, 844584);

    unsafe {
        use xz_embedded_sys as raw;
        raw::xz_crc32_init();
        raw::xz_crc64_init();
    }

    b.iter(|| {
        xz_emb_decompress(data, m.as_mut_slice());
        black_box(&mut m);
    });
}

pub fn xz_emb_decompress(compressed_data: &[u8], decompressed_data: &mut [u8]) {
    use xz_embedded_sys as raw;

    let state = unsafe { raw::xz_dec_init(raw::xz_mode::XZ_DYNALLOC, 1 << 26) };

    let mut buf = raw::xz_buf {
        _in: compressed_data.as_ptr(),
        in_size: compressed_data.len() as u64,
        in_pos: 0,

        out: decompressed_data.as_mut_ptr(),
        out_pos: 0,
        out_size: decompressed_data.len() as u64,
    };

    let ret = unsafe { raw::xz_dec_run(state, &mut buf) };
    assert_eq!(ret, raw::xz_ret::XZ_STREAM_END);
    unsafe {
        raw::xz_dec_end(state);
    }
}
