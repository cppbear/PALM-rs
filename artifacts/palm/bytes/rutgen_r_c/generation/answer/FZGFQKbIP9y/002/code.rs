// Answer 0

#[test]
#[should_panic(expected = "set_len out of bounds")]
fn test_set_len_panic() {
    struct TestBytesMut {
        ptr: NonNull<u8>,
        len: usize,
        cap: usize,
        data: *mut u8,
    }

    let mut buffer = TestBytesMut {
        ptr: NonNull::new_unchecked(std::ptr::null_mut()),
        len: 0,
        cap: 10,
        data: std::ptr::null_mut(),
    };

    unsafe {
        // Trying to set length greater than capacity to trigger panic
        buffer.set_len(15);
    }
}

#[test]
fn test_set_len_valid() {
    struct TestBytesMut {
        ptr: NonNull<u8>,
        len: usize,
        cap: usize,
        data: *mut u8,
    }

    let mut buffer = TestBytesMut {
        ptr: NonNull::new_unchecked(std::ptr::null_mut()),
        len: 0,
        cap: 10,
        data: std::ptr::null_mut(),
    };

    unsafe {
        // Set length to 5 which is less than capacity
        buffer.set_len(5);
        assert_eq!(buffer.len, 5);

        // Set length back to 10 which is equal to capacity
        buffer.set_len(10);
        assert_eq!(buffer.len, 10);
    }
}

#[test]
#[should_panic(expected = "set_len out of bounds")]
fn test_set_len_zero_capacity() {
    struct TestBytesMut {
        ptr: NonNull<u8>,
        len: usize,
        cap: usize,
        data: *mut u8,
    }

    let mut buffer = TestBytesMut {
        ptr: NonNull::new_unchecked(std::ptr::null_mut()),
        len: 0,
        cap: 0,
        data: std::ptr::null_mut(),
    };

    unsafe {
        // Attempt to set length greater than current capacity of 0
        buffer.set_len(1);
    }
}

