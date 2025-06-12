// Answer 0

#[test]
fn test_extend_from_slice_exceeding_capacity() {
    use bytes::BytesMut;

    struct TestBytesMut {
        inner: BytesMut,
    }

    impl TestBytesMut {
        fn new(capacity: usize) -> Self {
            TestBytesMut {
                inner: BytesMut::with_capacity(capacity),
            }
        }

        fn extend_from_slice(&mut self, extend: &[u8]) {
            self.inner.extend_from_slice(extend);
        }

        fn as_slice(&self) -> &[u8] {
            &self.inner[..]
        }
    }

    let mut buf = TestBytesMut::new(5);
    
    // Fill the initial capacity to ensure it gets resized
    buf.extend_from_slice(b"abc");

    // Now extend with a slice that exceeds the current capacity
    buf.extend_from_slice(b"defghijk"); // This will trigger resizing

    // Check the final result
    assert_eq!(b"abcdefghijk", buf.as_slice());
}

#[test]
#[should_panic]
fn test_extend_from_slice_panic_on_exceeding_capacity() {
    use bytes::BytesMut;

    struct TestBytesMut {
        inner: BytesMut,
    }

    impl TestBytesMut {
        fn new(capacity: usize) -> Self {
            TestBytesMut {
                inner: BytesMut::with_capacity(capacity),
            }
        }

        fn extend_from_slice(&mut self, extend: &[u8]) {
            self.inner.extend_from_slice(extend);
        }
    }

    let mut buf = TestBytesMut::new(0);

    // Attempting to extend with a large slice without enough initial capacity
    buf.extend_from_slice(&[0u8; usize::MAX]); // This will panic due to resizing issues
}

