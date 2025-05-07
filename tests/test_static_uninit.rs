use std::mem::MaybeUninit;
use std::sync::Mutex;
use xz4rust::XzStaticDecoder;

static DECODER: Mutex<MaybeUninit<XzStaticDecoder<{ xz4rust::DICT_SIZE_PROFILE_6 }>>> =
    Mutex::new(MaybeUninit::uninit());

#[test]
pub fn test_static_uninit() {
    let input = include_bytes!("../test_files/good-1-block_header-1.xz");
    let exp = include_bytes!("../test_files/good-1-block_header-1");

    let mut out_vec = Vec::new();

    let mut n = DECODER.lock().unwrap();
    let mut buf = [0u8; 1];
    let mut r = input.as_slice();
    unsafe {
        let g = XzStaticDecoder::<{ xz4rust::DICT_SIZE_PROFILE_6 }>::init_or_reset_at_address(
            n.as_mut_ptr().cast(),
            size_of::<XzStaticDecoder<{ xz4rust::DICT_SIZE_PROFILE_6 }>>(),
        );
        loop {
            let n = g.as_mut().unwrap().decode(&r[..1], &mut buf).unwrap();
            r = &r[n.input_consumed()..];
            let x = &buf.as_slice()[0..n.output_produced()];
            out_vec.extend_from_slice(x);
            if n.is_end_of_stream() {
                break;
            }
        }
    }

    assert_eq!(out_vec.as_slice(), exp);
}
