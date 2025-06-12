// Answer 0

#[test]
fn test_try_unsplit_capacity_zero() {
    let mut self_bytes = BytesMut::with_capacity(10);
    self_bytes.resize(5, 0);

    let other_bytes = BytesMut::with_capacity(0);

    match self_bytes.try_unsplit(other_bytes) {
        Ok(()) => panic!("Expected Err, got Ok"),
        Err(err) => assert_eq!(err.len(), 0, "Expected other to be an empty BytesMut"),
    }
}

#[test]
fn test_try_unsplit_non_contiguous() {
    struct KindNonArc;

    impl BytesMut {
        fn kind(&self) -> usize {
            // Return non-arc kind
            1 // KIND_VEC, for instance
        }
    }

    let mut self_bytes = BytesMut::with_capacity(10);
    self_bytes.resize(5, 0);

    // Create another BytesMut with some data
    let mut other_bytes = BytesMut::with_capacity(10);
    other_bytes.resize(3, 0);
    
    // Mimicking the condition where ptrs are not equal for the test
    unsafe {
        self_bytes.ptr = NonNull::new_unchecked(self_bytes.ptr.as_ptr().add(1)); // make ptr non-equal
    }

    match self_bytes.try_unsplit(other_bytes) {
        Ok(()) => panic!("Expected Err, got Ok"),
        Err(err) => assert_eq!(err.len(), 3, "Expected other to have length 3"),
    }
}

