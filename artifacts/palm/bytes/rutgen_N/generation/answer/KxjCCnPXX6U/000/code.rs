// Answer 0

#[test]
fn test_borrow_mut() {
    struct BytesMut {
        data: Vec<u8>,
    }

    impl BytesMut {
        fn new() -> Self {
            BytesMut { data: Vec::new() }
        }

        fn as_mut(&mut self) -> &mut [u8] {
            self.data.as_mut_slice()
        }

        fn borrow_mut(&mut self) -> &mut [u8] {
            self.as_mut()
        }
    }

    let mut bytes_mut = BytesMut::new();
    bytes_mut.data.extend_from_slice(&[1, 2, 3, 4, 5]);

    let slice: &mut [u8] = bytes_mut.borrow_mut();
    assert_eq!(slice, &[1, 2, 3, 4, 5]);

    slice[0] = 10;
    assert_eq!(bytes_mut.data, vec![10, 2, 3, 4, 5]);
}

#[test]
fn test_borrow_mut_empty() {
    struct BytesMut {
        data: Vec<u8>,
    }

    impl BytesMut {
        fn new() -> Self {
            BytesMut { data: Vec::new() }
        }

        fn as_mut(&mut self) -> &mut [u8] {
            self.data.as_mut_slice()
        }

        fn borrow_mut(&mut self) -> &mut [u8] {
            self.as_mut()
        }
    }

    let mut bytes_mut = BytesMut::new();
    let slice: &mut [u8] = bytes_mut.borrow_mut();
    assert!(slice.is_empty());
}

