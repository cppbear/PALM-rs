// Answer 0

#[test]
fn test_buf_new_with_non_empty_slice() {
    let mut data = [0u8; 10];
    let buf = Buf::new(&mut data);
    assert_eq!(buf.offset, 0);
    assert_eq!(buf.bytes.len(), 10);
}

#[test]
fn test_buf_new_with_empty_slice() {
    let mut data: [u8; 0] = [];
    let buf = Buf::new(&mut data);
    assert_eq!(buf.offset, 0);
    assert_eq!(buf.bytes.len(), 0);
}

#[test]
fn test_buf_new_with_large_slice() {
    let mut data = [0u8; 100];
    let buf = Buf::new(&mut data);
    assert_eq!(buf.offset, 0);
    assert_eq!(buf.bytes.len(), 100);
}

