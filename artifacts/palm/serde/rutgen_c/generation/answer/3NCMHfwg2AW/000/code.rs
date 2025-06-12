// Answer 0

#[test]
fn test_as_str_empty() {
    let mut bytes = [0u8; 0];
    let buf = Buf::new(&mut bytes);
    assert_eq!(buf.as_str(), "");
}

#[test]
fn test_as_str_non_empty() {
    let mut bytes = [b'h', b'e', b'l', b'l', b'o'];
    let mut buf = Buf::new(&mut bytes);
    buf.offset = 5; // Set offset to the length of "hello"
    assert_eq!(buf.as_str(), "hello");
}

#[test]
#[should_panic]
fn test_as_str_out_of_bounds() {
    let mut bytes = [b't', b'e', b's', b't'];
    let buf = Buf::new(&mut bytes);
    buf.offset = 5; // Out of bounds offset
    let _ = buf.as_str(); // This should panic due to out-of-bounds access
}

