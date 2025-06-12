// Answer 0

#[test]
fn test_put_slice_panic_not_enough_capacity() {
    let mut dst = [0; 5];
    let mut buf = &mut dst[..];

    // Source is larger than the buffer remaining capacity
    let src = b"hello, world!";

    let result = std::panic::catch_unwind(|| {
        buf.put_slice(src);
    });

    assert!(result.is_err());
}

#[test]
fn test_put_slice_exact_fit() {
    let mut dst = [0; 5];
    let mut buf = &mut dst[..];

    let src = b"hello";

    buf.put_slice(src);

    assert_eq!(b"hello", &dst);
    assert_eq!(buf.remaining_mut(), 0); // No remaining space
}

#[test]
fn test_put_slice_partial_fit() {
    let mut dst = [0; 6];
    let mut buf = &mut dst[..];

    let src = b"hi";

    buf.put_slice(src);

    assert_eq!(b"hi\0\0\0", &dst);
    assert_eq!(buf.remaining_mut(), 4); // 4 bytes remaining
}

#[test]
fn test_put_slice_multiple_chunks() {
    let mut dst = [0; 10];
    let mut buf = &mut dst[..];

    let src = b"hellohello";

    buf.put_slice(src);

    assert_eq!(b"hellohell\0", &dst);
    assert_eq!(buf.remaining_mut(), 1); // 1 byte remaining
}

