// Answer 0

#[test]
fn test_remaining() {
    struct BytesMut {
        data: Vec<u8>,
    }

    impl BytesMut {
        fn new() -> Self {
            BytesMut { data: Vec::new() }
        }

        fn len(&self) -> usize {
            self.data.len()
        }

        fn remaining(&self) -> usize {
            self.len()
        }
    }

    let b = BytesMut::new();
    assert_eq!(b.remaining(), 0);

    let mut b_with_data = BytesMut::new();
    b_with_data.data.extend_from_slice(&[1, 2, 3, 4, 5]);
    assert_eq!(b_with_data.remaining(), 5);
}

