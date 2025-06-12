// Answer 0

#[test]
fn test_shallow_clone_promote_to_shared() {
    // Defining a simple struct to initialize BytesMut
    struct TestBytesMut {
        bytes: BytesMut,
    }

    // Initialize with a specific capacity
    let mut test_bytes = TestBytesMut {
        bytes: BytesMut::with_capacity(16),
    };

    // Set the data field to indicate that it's a kind that's not ARC
    // Manually setting it to KIND_VEC for the purpose of the test
    unsafe {
        let shared_ptr = Box::new(Shared {
            vec: Vec::new(),
            original_capacity_repr: 0,
            ref_count: AtomicUsize::new(1),
        });

        test_bytes.bytes.data = Box::into_raw(shared_ptr);
    }

    // Perform the unsafe operation
    unsafe {
        let clone = test_bytes.bytes.shallow_clone();
        // Validate the operation did not panic
        assert!(clone.kind() != KIND_ARC);
        // Check if the original and clone are distinct pointers and are in the right state
        assert_ne!(test_bytes.bytes.data, clone.data);
        assert_eq!(test_bytes.bytes.len(), clone.len());
    }
}

#[test]
#[should_panic]
fn test_shallow_clone_increments_shared_ref_count() {
    // Defining a struct to check panic conditions
    struct TestBytesMut {
        bytes: BytesMut,
    }

    let mut test_bytes = TestBytesMut {
        bytes: BytesMut::with_capacity(16),
    };

    // Set the data field to indicate it's an ARC kind
    unsafe {
        let shared_ptr = Box::new(Shared {
            vec: Vec::new(),
            original_capacity_repr: 0,
            ref_count: AtomicUsize::new(usize::MAX), // Set to max to force a panic
        });

        test_bytes.bytes.data = Box::into_raw(shared_ptr);
    }

    // This will panic due to incrementing over the maximum value
    unsafe {
        test_bytes.bytes.shallow_clone();
    }
}

