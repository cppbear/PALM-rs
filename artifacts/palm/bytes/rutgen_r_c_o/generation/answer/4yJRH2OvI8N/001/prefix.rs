// Answer 0

#[test]
fn test_first_ref_non_empty_buf() {
    let buf1 = Vec::<u8>::from(&b"hello"[..]);
    let buf2 = Vec::<u8>::from(&b""[..]);
    let chain = Chain::new(buf1, buf2);
    chain.first_ref();
}

#[test]
fn test_first_ref_non_empty_buf_with_non_empty_second() {
    let buf1 = Vec::<u8>::from(&b"hello"[..]);
    let buf2 = Vec::<u8>::from(&b"world"[..]);
    let chain = Chain::new(buf1, buf2);
    chain.first_ref();
}

#[test]
fn test_first_ref_large_non_empty_buf() {
    let buf1 = vec![0u8; 1024];
    let buf2 = Vec::<u8>::from(&b""[..]);
    let chain = Chain::new(buf1, buf2);
    chain.first_ref();
}

#[test]
fn test_first_ref_non_empty_buf_large_second() {
    let buf1 = Vec::<u8>::from(&b"hello"[..]);
    let buf2 = vec![0u8; 1024];
    let chain = Chain::new(buf1, buf2);
    chain.first_ref();
}

#[test]
fn test_first_ref_multiple_non_empty_bufs() {
    let buf1 = Vec::<u8>::from(&b"abcdef"[..]);
    let buf2 = Vec::<u8>::from(&b"ghijkl"[..]);
    let chain = Chain::new(buf1, buf2);
    chain.first_ref();
}

#[test]
fn test_first_ref_empty_second_buf() {
    let buf1 = Vec::<u8>::from(&b"x"[..]);
    let buf2 = Vec::<u8>::from(&b""[..]);
    let chain = Chain::new(buf1, buf2);
    chain.first_ref();
}

