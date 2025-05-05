use xz4rust::{XzDecoder, XzNextBlockResult};

fn run_test(seed: [usize; 32]) {
    let data = include_bytes!("../test_files/java_native_utils_amd64.so2.xz");
    let expect = include_bytes!("../test_files/java_native_utils_amd64.so");

    let mut dec = XzDecoder::in_heap();
    let mut out = Vec::new();
    out.resize(expect.len(), 0);
    let mut sl = data.as_slice();
    let mut sl2 = out.as_mut_slice();
    let mut count = 0;
    let mut count2 = 7;
    loop {
        count += 1;
        count2 += 1;
        count %= seed.len();
        count2 %= seed.len();

        let sl2len = sl2.len();

        match dec
            .decode(
                &sl[..seed[count].min(sl.len())],
                &mut sl2[..seed[count2].min(sl2len)],
            )
            .expect("failed to decode")
        {
            XzNextBlockResult::NeedMoreData(inp, outp) => {
                sl = &sl[inp..];
                sl2 = &mut sl2[outp..];
            }
            XzNextBlockResult::EndOfStream(_, _) => {
                break;
            }
        }
    }

    assert_eq!(out.as_slice(), expect.as_slice());
}

#[test]
fn cb_t1() {
    //Chosen randomly by smashing numpad and adding commas randomly.
    run_test([
        3usize, 7, 4, 2, 1, 2, 8, 1, 5, 7, 1, 11, 1, 3, 4, 2, 1, 9, 2, 1, 3, 2, 4, 6, 3, 3, 1, 8,
        7, 3, 1, 2,
    ])
}

#[test]
fn cb_t2() {
    // Compared to t1 this contains one "chunk" which is large enough for a lzma operation (21-63 bytes)
    // and should test the temp buffer behavior if the temp buffer is partially filled in lzma_main.
    run_test([
        3usize, 7, 4, 2, 1, 2, 8, 1, 5, 7, 1, 11, 65, 3, 4, 2, 1, 9, 2, 1, 3, 2, 4, 6, 3, 3, 1, 8,
        7, 3, 1, 2,
    ])
}
