// Answer 0

#[test]
fn test_remaining_mut_zero_length() {
    struct BytesMut {
        data: Vec<u8>,
    }

    impl BytesMut {
        fn len(&self) -> usize {
            self.data.len()
        }

        fn remaining_mut(&self) -> usize {
            usize::MAX - self.len()
        }
    }

    let bytes = BytesMut { data: vec![] };
    assert_eq!(bytes.remaining_mut(), usize::MAX);
}

#[test]
fn test_remaining_mut_small_length() {
    struct BytesMut {
        data: Vec<u8>,
    }

    impl BytesMut {
        fn len(&self) -> usize {
            self.data.len()
        }

        fn remaining_mut(&self) -> usize {
            usize::MAX - self.len()
        }
    }

    let bytes = BytesMut { data: vec![1, 2, 3] };
    assert_eq!(bytes.remaining_mut(), usize::MAX - 3);
}

#[test]
fn test_remaining_mut_large_length() {
    struct BytesMut {
        data: Vec<u8>,
    }

    impl BytesMut {
        fn len(&self) -> usize {
            self.data.len()
        }

        fn remaining_mut(&self) -> usize {
            usize::MAX - self.len()
        }
    }

    let bytes = BytesMut { data: vec![0; usize::MAX as usize] };
    assert_eq!(bytes.remaining_mut(), 0); // This will not panic but returns 0 as the length is maximum
}

#[should_panic]
fn test_remaining_mut_exceeding_capacity() {
    struct BytesMut {
        data: Vec<u8>,
    }

    impl BytesMut {
        fn len(&self) -> usize {
            self.data.len()
        }

        fn remaining_mut(&self) -> usize {
            usize::MAX - self.len()
        }
    }

    let bytes = BytesMut { data: vec![0; (usize::MAX - 1) as usize] }; // Creates a vector very close to max size but valid
    // The next length should provoke a panic due to usize::MAX- calculated value.
    let _ = bytes.remaining_mut(); // Expect this to panic in the case of larger sizes leading to negative usize computation conceptually
}

