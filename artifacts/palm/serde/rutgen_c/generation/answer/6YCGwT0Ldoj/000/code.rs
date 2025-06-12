// Answer 0

#[test]
fn test_new_buf_with_non_empty_slice() {
    let mut bytes = [0u8; 10];
    let buf = Buf::new(&mut bytes);
    assert_eq!(buf.offset, 0);
}

#[test]
fn test_new_buf_with_empty_slice() {
    let mut bytes: &[u8] = &mut [];
    let buf = Buf::new(bytes);
    assert_eq!(buf.offset, 0);
}

