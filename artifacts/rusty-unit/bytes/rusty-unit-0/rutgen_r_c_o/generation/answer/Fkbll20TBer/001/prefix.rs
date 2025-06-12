// Answer 0

#[test]
fn test_from_static_empty() {
    let b = Bytes::from_static(b"");
}

#[test]
fn test_from_static_hello() {
    let b = Bytes::from_static(b"hello");
}

#[test]
fn test_from_static_full_capacity() {
    let b = Bytes::from_static(b"a".repeat(1024).as_slice());
}

#[test]
fn test_from_static_mixed_content() {
    let b = Bytes::from_static(b"mixed content with symbols: !@#$%^&*()");
}

#[test]
#[should_panic]
fn test_from_static_invalid_memory() {
    let invalid: &'static [u8] = core::mem::transmute::<&'static [u8], &'static [u8]>(b"invalid");
    let b = Bytes::from_static(invalid);
}

