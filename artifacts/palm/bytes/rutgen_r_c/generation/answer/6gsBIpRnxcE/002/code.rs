// Answer 0

#[test]
#[should_panic(expected = "internal: inc_start out of bounds")]
fn test_inc_start_panics_when_by_greater_than_len() {
    let mut bytes = Bytes::new(); // Initialized with len = 0
    unsafe {
        // Attempt to call inc_start with a value greater than the current length
        bytes.inc_start(1); // This should panic
    }
}

#[test]
#[should_panic(expected = "internal: inc_start out of bounds")]
fn test_inc_start_panics_when_by_equals_len() {
    let bytes = Bytes::from_static(b"test"); // Initialized with len = 4
    unsafe {
        // Attempt to call inc_start with a value equal to the current length
        bytes.inc_start(4); // This should panic
    }
}

#[test]
fn test_inc_start_does_not_panic_when_by_is_zero() {
    let mut bytes = Bytes::from_static(b"test"); // Initialized with len = 4
    unsafe {
        bytes.inc_start(0); // This should not panic
        assert_eq!(bytes.len(), 4);
    }
}

#[test]
fn test_inc_start_decreases_length_correctly() {
    let mut bytes = Bytes::from_static(b"example"); // Initialized with len = 7
    unsafe {
        bytes.inc_start(3); // Decrease length by 3
        assert_eq!(bytes.len(), 4); // Length should now be 4
        assert_eq!(bytes.ptr as usize, (bytes.ptr as usize + 3)); // Pointer should be moved forward by 3
    }
}

