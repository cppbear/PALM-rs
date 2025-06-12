// Answer 0

#[test]
fn test_copy_to_slice_normal_case() {
    let mut buf: &[u8] = &b"hello world"[..];
    let mut dst = [0; 5];

    buf.copy_to_slice(&mut dst);
    
    assert_eq!(&b"hello"[..], &dst);
    assert_eq!(6, buf.remaining());
}

#[test]
#[should_panic]
fn test_copy_to_slice_insufficient_bytes() {
    let mut buf: &[u8] = &b"hi"[..];
    let mut dst = [0; 3];

    buf.copy_to_slice(&mut dst);
}

#[test]
fn test_copy_to_slice_exact_fit() {
    let mut buf: &[u8] = &b"hello"[..];
    let mut dst = [0; 5];

    buf.copy_to_slice(&mut dst);
    
    assert_eq!(&b"hello"[..], &dst);
    assert_eq!(0, buf.remaining());
}

#[test]
fn test_copy_to_slice_empty_buffer() {
    let mut buf: &[u8] = &b""[..];
    let mut dst = [0; 0];

    buf.copy_to_slice(&mut dst);
    
    assert_eq!(&[][..], &dst);
    assert_eq!(0, buf.remaining());
}

#[test]
#[should_panic]
fn test_copy_to_slice_overflow() {
    let mut buf: &[u8] = &b"abcdef"[..];
    let mut dst = [0; 10];

    buf.copy_to_slice(&mut dst);
}

