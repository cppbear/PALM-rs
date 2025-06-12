// Answer 0

#[test]
fn test_put_slice_empty_source() {
    let mut bytes_mut = BytesMut::new();
    let src: &[u8] = &[];
    bytes_mut.put_slice(src);
}

#[test]
fn test_put_slice_small_source() {
    let mut bytes_mut = BytesMut::with_capacity(10);
    let src: &[u8] = &[1, 2, 3];
    bytes_mut.put_slice(src);
}

#[test]
fn test_put_slice_large_source() {
    let mut bytes_mut = BytesMut::with_capacity(32);
    let src: &[u8] = &[4; 20];
    bytes_mut.put_slice(src);
}

#[test]
fn test_put_slice_exact_capacity() {
    let mut bytes_mut = BytesMut::with_capacity(17);
    let src: &[u8] = &[5; 17];
    bytes_mut.put_slice(src);
}

#[test]
#[should_panic]
fn test_put_slice_exceeding_capacity() {
    let mut bytes_mut = BytesMut::with_capacity(5);
    let src: &[u8] = &[6; 10];
    bytes_mut.put_slice(src);
}

#[test]
fn test_put_slice_null_pointer() {
    let mut bytes_mut = BytesMut::new();
    let src: *const u8 = std::ptr::null();
    let src_slice: &[u8] = unsafe { std::slice::from_raw_parts(src, 0) }; // This should be a safe usage
    bytes_mut.put_slice(src_slice);
}

