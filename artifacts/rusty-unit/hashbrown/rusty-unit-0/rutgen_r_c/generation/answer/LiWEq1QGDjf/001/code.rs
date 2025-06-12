// Answer 0

#[test]
fn test_next_impl_yields_index() {
    struct TestGroup {
        current_group: BitMaskIter,
        group_first_index: usize,
        ctrl: NonNull<u8>,
        items: usize,
    }

    // Create a sample BitMask and initialize the BitMaskIter
    let bitmask = BitMask(0b101); // This should yield indices 0 and 2
    let bitmask_iter = BitMaskIter(bitmask);
    let ctrl_ptr = NonNull::new(Box::into_raw(Box::new([0u8; 32]))).unwrap(); // Dummy memory for control bytes

    let mut test_group = TestGroup {
        current_group: bitmask_iter,
        group_first_index: 0,
        ctrl: ctrl_ptr,
        items: 3, // Number of items in this simulation
    };

    unsafe {
        assert_eq!(test_group.current_group.next(), Some(0)); // First call should yield 0
        assert_eq!(test_group.group_first_index, 0);

        test_group.group_first_index += 1; // Manually increment for next index

        assert_eq!(test_group.current_group.next(), Some(2)); // Second call should yield 2
        assert_eq!(test_group.group_first_index, 1);
    }
}

#[test]
fn test_next_impl_handles_exhaustion() {
    struct TestGroup {
        current_group: BitMaskIter,
        group_first_index: usize,
        ctrl: NonNull<u8>,
        items: usize,
    }

    // Create a sample BitMask with no indices left
    let bitmask = BitMask(0); // No bits set
    let bitmask_iter = BitMaskIter(bitmask);
    let ctrl_ptr = NonNull::new(Box::into_raw(Box::new([0u8; 32]))).unwrap(); // Dummy memory for control bytes

    let mut test_group = TestGroup {
        current_group: bitmask_iter,
        group_first_index: 0,
        ctrl: ctrl_ptr,
        items: 0,
    };

    unsafe {
        assert_eq!(test_group.current_group.next(), None); // Should return None
    }
}

