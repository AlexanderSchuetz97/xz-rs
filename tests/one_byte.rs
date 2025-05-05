use xz4rust::{XzDecoder, XzError, XzNextBlockResult};

#[test]
fn one_byte_output() {
    let data = include_bytes!("../test_files/java_native_utils_amd64.so2.xz");
    let mut dec = XzDecoder::in_heap();
    let mut out = Vec::new();
    let mut sl = data.as_slice();
    loop {
        let mut buf = [0];
        match dec.decode(sl, buf.as_mut_slice()).expect("failed to decode") {
            XzNextBlockResult::NeedMoreData(inp, outp) => {
                sl = &sl[inp..];
                if outp != 0 {
                    assert_eq!(outp, 1);
                    out.push(buf[0])
                }
            }
            XzNextBlockResult::EndOfStream(_, outp) => {
                if outp != 0 {
                    assert_eq!(outp, 1);
                    out.push(buf[0])
                }
                break;
            }
        }
    }

    let expect = include_bytes!("../test_files/java_native_utils_amd64.so");
    assert_eq!(out.as_slice(), expect.as_slice());
}

#[test]
fn one_byte_input_output() {
    let data = include_bytes!("../test_files/java_native_utils_amd64.so2.xz");
    let mut dec = XzDecoder::in_heap();
    let mut out = Vec::new();
    let mut sl = data.as_slice();
    loop {
        let mut buf = [0];
        match dec.decode(&sl[..1], buf.as_mut_slice()).expect("failed to decode") {
            XzNextBlockResult::NeedMoreData(inp, outp) => {
                sl = &sl[inp..];
                if outp != 0 {
                    assert_eq!(outp, 1);
                    out.push(buf[0])
                }
            }
            XzNextBlockResult::EndOfStream(_, outp) => {
                if outp != 0 {
                    assert_eq!(outp, 1);
                    out.push(buf[0])
                }
                break;
            }
        }
    }

    let expect = include_bytes!("../test_files/java_native_utils_amd64.so");
    assert_eq!(out.as_slice(), expect.as_slice());
}

#[test]
fn one_byte_input() {
    let data = include_bytes!("../test_files/java_native_utils_amd64.so2.xz");
    let expect = include_bytes!("../test_files/java_native_utils_amd64.so");
    let mut dec = XzDecoder::in_heap();
    let mut out = Vec::new();
    out.resize(expect.len(), 0);
    let mut out_pos = 0;
    let mut sl = data.as_slice();
    loop {
        match dec.decode(&sl[..1], &mut out.as_mut_slice()[out_pos..]).expect("failed to decode") {
            XzNextBlockResult::NeedMoreData(inp, outp) => {
                sl = &sl[inp..];
                out_pos+=outp;
            }
            XzNextBlockResult::EndOfStream(_, outp) => {
                out_pos+=outp;
                out.truncate(out_pos);
                break;
            }
        }
    }

    assert_eq!(out.as_slice(), expect.as_slice());
}


#[test]
fn one_byte_input_output_bad() {
    let data = include_bytes!("../test_files/bad-1-lzma2-7.xz");
    let mut dec = XzDecoder::in_heap();
    let mut out = Vec::new();
    let mut sl = data.as_slice();
    loop {
        let mut buf = [0];
        match dec.decode(&sl[..1], buf.as_mut_slice()) {
            Ok(XzNextBlockResult::NeedMoreData(inp, outp)) => {
                sl = &sl[inp..];
                if outp != 0 {
                    assert_eq!(outp, 1);
                    out.push(buf[0])
                }
            }
            Ok(XzNextBlockResult::EndOfStream(_, outp)) => {
                if outp != 0 {
                    assert_eq!(outp, 1);
                    out.push(buf[0])
                }
                panic!("Error expected");
            }
            Err(e) => {
                assert_eq!(e, XzError::CorruptedDataInLzma);
                break;
            }
        }
    }
}