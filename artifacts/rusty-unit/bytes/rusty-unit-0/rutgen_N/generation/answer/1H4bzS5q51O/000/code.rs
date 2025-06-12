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
fn test_copy_to_slice_insufficient_buf() {
    let mut buf: &[u8] = &b"hi"[..];
    let mut dst = [0; 5];
    
    buf.copy_to_slice(&mut dst);
}

