// Answer 0

#[test]
fn test_write_exponent3_k_10_no_sign() {
    let mut buf: [u8; 3] = [0; 3];
    let result = unsafe { write_exponent3(10, buf.as_mut_ptr()) };

    assert_eq!(result, 2);
    assert_eq!(&buf[..2], b"10");
}

#[test]
fn test_write_exponent3_k_0_no_sign() {
    let mut buf: [u8; 3] = [0; 3];
    let result = unsafe { write_exponent3(0, buf.as_mut_ptr()) };

    assert_eq!(result, 1);
    assert_eq!(buf[0], b'0');
}

#[test]
fn test_write_exponent3_k_99_no_sign() {
    let mut buf: [u8; 3] = [0; 3];
    let result = unsafe { write_exponent3(99, buf.as_mut_ptr()) };

    assert_eq!(result, 2);
    assert_eq!(&buf[..2], b"99");
}

