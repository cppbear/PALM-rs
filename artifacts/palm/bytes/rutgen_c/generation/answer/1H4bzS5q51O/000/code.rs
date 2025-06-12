// Answer 0

#[test]
fn test_copy_to_slice_success() {
    let mut buf: &[u8] = &b"hello world"[..];
    let mut dst = [0; 5];

    buf.copy_to_slice(&mut dst);
    assert_eq!(&b"hello"[..], &dst);
    assert_eq!(6, buf.remaining());
}

#[test]
#[should_panic]
fn test_copy_to_slice_not_enough_remaining() {
    let mut buf: &[u8] = &b"hello"[..];
    let mut dst = [0; 6];

    buf.copy_to_slice(&mut dst);
}

#[test]
fn test_copy_to_slice_exact_fit() {
    let mut buf: &[u8] = &b"rust"[..];
    let mut dst = [0; 4];

    buf.copy_to_slice(&mut dst);
    assert_eq!(&b"rust"[..], &dst);
    assert_eq!(0, buf.remaining());
}

#[test]
fn test_copy_to_slice_empty_buffer() {
    let mut buf: &[u8] = &b""[..];
    let mut dst = [0; 0];

    buf.copy_to_slice(&mut dst);
    assert_eq!(&b""[..], &dst);
    assert_eq!(0, buf.remaining());
}

