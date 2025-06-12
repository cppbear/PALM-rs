// Answer 0

#[test]
fn test_put_slice_exact_fit() {
    let mut dst: [core::mem::MaybeUninit<u8>; 6] = Default::default();
    let buf: &mut [core::mem::MaybeUninit<u8>] = &mut dst;

    // Fill with enough data to exactly match the available capacity
    let src = b"hello";

    buf.put_slice(src);

    let result: Vec<u8> = unsafe { core::slice::from_raw_parts(buf.as_mut_ptr() as *const u8, buf.len()).to_vec() };

    assert_eq!(result, b"hello\0");
}

#[test]
#[should_panic(expected = "advance out of bounds")]
fn test_put_slice_exceed_capacity() {
    let mut dst: [core::mem::MaybeUninit<u8>; 5] = Default::default();
    let buf: &mut [core::mem::MaybeUninit<u8>] = &mut dst;

    let src = b"hello world"; // Exceeds available capacity

    buf.put_slice(src);
}

#[test]
fn test_put_slice_partial_fit() {
    let mut dst: [core::mem::MaybeUninit<u8>; 10] = Default::default();
    let buf: &mut [core::mem::MaybeUninit<u8>] = &mut dst;

    let src = b"hello world";

    buf.put_slice(&src[..5]); // Only putting "hello", which fits

    let result: Vec<u8> = unsafe { core::slice::from_raw_parts(buf.as_mut_ptr() as *const u8, buf.len()).to_vec() };

    assert_eq!(result, b"hello\0\0\0\0\0\0\0\0\0");
}

#[test]
fn test_put_slice_empty_source() {
    let mut dst: [core::mem::MaybeUninit<u8>; 6] = Default::default();
    let buf: &mut [core::mem::MaybeUninit<u8>] = &mut dst;

    let src: &[u8] = b""; // Empty slice

    buf.put_slice(src); // Should not panic

    let result: Vec<u8> = unsafe { core::slice::from_raw_parts(buf.as_mut_ptr() as *const u8, buf.len()).to_vec() };

    assert_eq!(result, b"\0\0\0\0\0\0");
}

#[test]
#[should_panic(expected = "advance out of bounds")]
fn test_put_slice_panic_on_too_large_slice() {
    let mut dst: [core::mem::MaybeUninit<u8>; 5] = Default::default();
    let buf: &mut [core::mem::MaybeUninit<u8>] = &mut dst;

    let src = b"abcdef"; // Too large for the destination

    buf.put_slice(src);
}

