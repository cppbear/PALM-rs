// Answer 0

#[test]
fn test_advance_unchecked_with_zero_count() {
    let mut buffer = BytesMut::with_capacity(10);
    unsafe {
        buffer.advance_unchecked(0);
    }
}

#[test]
fn test_advance_unchecked_with_cap_count() {
    let mut buffer = BytesMut::with_capacity(10);
    unsafe {
        buffer.reserve(10);
        buffer.set_len(10);
        buffer.ptr = NonNull::new(buffer.spare_capacity_mut().as_mut_ptr() as *mut u8).unwrap();
        buffer.extend_from_slice(&[0u8; 10]);
        buffer.advance_unchecked(buffer.cap);
    }
}

#[test]
fn test_advance_unchecked_with_max_vec_pos() {
    let mut buffer = BytesMut::with_capacity(10);
    unsafe {
        buffer.reserve(10);
        buffer.set_len(10);
        buffer.ptr = NonNull::new(buffer.spare_capacity_mut().as_mut_ptr() as *mut u8).unwrap();
        buffer.extend_from_slice(&[0u8; 10]);
        buffer.advance_unchecked(buffer.len());
    }
}

