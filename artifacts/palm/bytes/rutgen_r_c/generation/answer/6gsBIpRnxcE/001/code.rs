// Answer 0

#[test]
fn test_inc_start_with_exact_length() {
    let mut bytes = unsafe {
        Bytes::with_vtable(
            b"Test"[..].as_ptr(),
            4,
            AtomicPtr::new(ptr::null_mut()),
            &STATIC_VTABLE,
        )
    };
    let initial_len = bytes.len();
    let by = initial_len; // Constraint: self.len >= by and self.len == by

    unsafe {
        bytes.inc_start(by);
    }

    assert_eq!(bytes.len(), 0); // After inc_start, length should be 0
}

#[test]
#[should_panic(expected = "internal: inc_start out of bounds")]
fn test_inc_start_with_exceeding_length() {
    let mut bytes = unsafe {
        Bytes::with_vtable(
            b"Test"[..].as_ptr(),
            4,
            AtomicPtr::new(ptr::null_mut()),
            &STATIC_VTABLE,
        )
    };

    let exceeding_by = 5; // Constraint: self.len >= by is false

    unsafe {
        bytes.inc_start(exceeding_by); // This should panic
    }
}

