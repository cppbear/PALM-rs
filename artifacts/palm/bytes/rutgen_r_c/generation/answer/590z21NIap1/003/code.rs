// Answer 0

#[test]
fn test_advance_unchecked_zero_count() {
    let mut bytes_mut = unsafe { BytesMut::with_capacity(10) };
    unsafe {
        bytes_mut.advance_unchecked(0);
    }
    assert_eq!(bytes_mut.len(), 10);
}

#[test]
fn test_advance_unchecked_exact_capacity() {
    let mut bytes_mut = unsafe { BytesMut::with_capacity(10) };
    unsafe {
        bytes_mut.advance_unchecked(10);
    }
    assert_eq!(bytes_mut.len(), 0);
}

#[test]
#[should_panic(expected = "internal: set_start out of bounds")]
fn test_advance_unchecked_exceeding_vec_pos() {
    let mut bytes_mut = unsafe { BytesMut::with_capacity(5) };
    unsafe {
        // Simulating a condition where position exceeds MAX_VEC_POS
        bytes_mut.set_vec_pos(MAX_VEC_POS); // This must happen before advance_unchecked
        bytes_mut.advance_unchecked(10); // This should trigger panic
    }
}

#[test]
fn test_advance_unchecked_normal_case() {
    let mut bytes_mut = unsafe { BytesMut::with_capacity(20) };
    unsafe {
        bytes_mut.advance_unchecked(5);
    }
    assert_eq!(bytes_mut.len(), 15);
}

