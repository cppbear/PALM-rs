// Answer 0

#[test]
fn test_copy_to_slice_success() {
    let mut buf: &mut [u8] = &mut b"hello world"[..];
    let mut dst = [0; 5];
    buf.copy_to_slice(&mut dst);
    assert_eq!(&b"hello"[..], &dst);
}

#[test]
#[should_panic(expected = "TryGetError { requested: 10, available: 5 }]")]
fn test_copy_to_slice_insufficient_bytes() {
    let mut buf: &mut [u8] = &mut b"hello"[..];
    let mut dst = [0; 10];
    buf.copy_to_slice(&mut dst);
}

#[test]
fn test_copy_to_slice_exact_fit() {
    let mut buf: &mut [u8] = &mut b"hello"[..];
    let mut dst = [0; 5];
    buf.copy_to_slice(&mut dst);
    assert_eq!(&b"hello"[..], &dst);
    assert_eq!(0, buf.remaining());
}

#[test]
fn test_copy_to_slice_empty_buffer() {
    let mut buf: &mut [u8] = &mut b""[..];
    let mut dst = [0; 0];
    buf.copy_to_slice(&mut dst);
    assert_eq!(0, buf.remaining());
}

#[test]
#[should_panic(expected = "TryGetError { requested: 5, available: 0 }]")]
fn test_copy_to_slice_panic_empty_dst_non_empty_buf() {
    let mut buf: &mut [u8] = &mut b"data"[..];
    let mut dst = [0; 5];
    buf.copy_to_slice(&mut dst);
}

