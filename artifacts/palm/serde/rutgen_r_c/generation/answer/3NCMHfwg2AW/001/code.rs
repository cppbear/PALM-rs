// Answer 0

#[test]
fn test_as_str_valid() {
    let mut buffer = [104, 101, 108, 108, 111]; // "hello" in UTF-8
    let buf = Buf::new(&mut buffer);
    assert_eq!(buf.as_str(), "hello");
}

#[test]
fn test_as_str_empty() {
    let mut buffer: [u8; 5] = [0; 5]; // empty slice
    let mut buf = Buf::new(&mut buffer);
    buf.offset = 0; // starting with empty
    assert_eq!(buf.as_str(), "");
}

#[test]
#[should_panic]
fn test_as_str_panic() {
    let mut buffer = [104, 101, 108, 108, 111]; // "hello" in UTF-8
    let mut buf = Buf::new(&mut buffer);
    buf.offset = 6; // out of bounds offset to trigger panic
    let _result = buf.as_str(); // this should panic
}

