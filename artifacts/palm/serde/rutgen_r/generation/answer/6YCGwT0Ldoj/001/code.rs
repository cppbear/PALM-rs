// Answer 0

#[test]
fn test_new_with_valid_byte_slice() {
    let mut bytes: [u8; 10] = [0; 10];
    let buf = new(&mut bytes);
    assert_eq!(buf.bytes.len(), 10);
    assert_eq!(buf.offset, 0);
}

#[test]
fn test_new_with_empty_byte_slice() {
    let mut bytes: [u8; 0] = [];
    let buf = new(&mut bytes);
    assert_eq!(buf.bytes.len(), 0);
    assert_eq!(buf.offset, 0);
}

#[should_panic]
fn test_new_with_null_byte_slice() {
    let bytes: *mut u8 = std::ptr::null_mut();
    let _buf = new(unsafe { &mut *bytes }); // should panic
}

#[test]
fn test_new_with_large_byte_slice() {
    let mut bytes: [u8; 100] = [0; 100];
    let buf = new(&mut bytes);
    assert_eq!(buf.bytes.len(), 100);
    assert_eq!(buf.offset, 0);
}

