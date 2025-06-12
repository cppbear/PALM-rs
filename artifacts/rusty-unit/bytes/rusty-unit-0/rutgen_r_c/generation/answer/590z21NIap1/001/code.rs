// Answer 0

#[test]
fn test_advance_unchecked_increments_pos_within_capacity() {
    let mut bytes_mut = unsafe { BytesMut::with_capacity(10) };
    let initial_ptr = bytes_mut.ptr;
    bytes_mut.len = 10;
    bytes_mut.cap = 10;
    let count = 5;
    unsafe { bytes_mut.advance_unchecked(count) };

    assert_eq!(bytes_mut.len, 5);
    assert_eq!(bytes_mut.cap, 5);
    assert_ne!(bytes_mut.ptr, initial_ptr); 
    assert!(bytes_mut.ptr.as_ptr() > initial_ptr.as_ptr());
}

#[test]
fn test_advance_unchecked_zero_count_is_no_op() {
    let mut bytes_mut = unsafe { BytesMut::with_capacity(10) };
    bytes_mut.len = 10;
    bytes_mut.cap = 10;
    let initial_len = bytes_mut.len;
    let initial_cap = bytes_mut.cap;

    unsafe { bytes_mut.advance_unchecked(0) };

    assert_eq!(bytes_mut.len, initial_len);
    assert_eq!(bytes_mut.cap, initial_cap);
}

#[test]
fn test_advance_unchecked_exceeding_vec_pos_promotes_to_shared() {
    let mut bytes_mut = unsafe { BytesMut::with_capacity(10) };
    bytes_mut.len = 10;
    bytes_mut.cap = 10;
    let large_count = 10; // Should trigger promotion
    unsafe { bytes_mut.advance_unchecked(large_count) };

    // Ensure that the internal state indicates promotion to shared
    assert!(bytes_mut.kind() == KIND_ARC); // Assuming how kind is checked for ARC
}

#[test]
#[should_panic(expected = "internal: set_start out of bounds")]
fn test_advance_unchecked_out_of_bounds() {
    let mut bytes_mut = unsafe { BytesMut::with_capacity(10) };
    bytes_mut.len = 10;
    bytes_mut.cap = 10;
    let count = 15; // Out of bounds
    unsafe { bytes_mut.advance_unchecked(count) };
}

