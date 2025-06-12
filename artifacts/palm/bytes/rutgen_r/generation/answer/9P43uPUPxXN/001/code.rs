// Answer 0

#[test]
fn test_empty_bytes_mut() {
    struct BytesMut {
        len: usize,
        capacity: usize,
    }

    impl BytesMut {
        fn with_capacity(capacity: usize) -> Self {
            BytesMut { len: 0, capacity }
        }

        fn is_empty(&self) -> bool {
            self.len == 0
        }
    }

    let b = BytesMut::with_capacity(64);
    assert!(b.is_empty());
}

#[test]
fn test_non_empty_bytes_mut() {
    struct BytesMut {
        len: usize,
        capacity: usize,
    }

    impl BytesMut {
        fn with_capacity(capacity: usize) -> Self {
            BytesMut { len: capacity, capacity }
        }

        fn is_empty(&self) -> bool {
            self.len == 0
        }
    }

    let b = BytesMut::with_capacity(64);
    assert!(!b.is_empty());
}

#[test]
fn test_zero_length_bytes_mut() {
    struct BytesMut {
        len: usize,
        capacity: usize,
    }

    impl BytesMut {
        fn with_capacity(_capacity: usize) -> Self {
            BytesMut { len: 0, capacity: 0 }
        }

        fn is_empty(&self) -> bool {
            self.len == 0
        }
    }

    let b = BytesMut::with_capacity(0);
    assert!(b.is_empty());
}

