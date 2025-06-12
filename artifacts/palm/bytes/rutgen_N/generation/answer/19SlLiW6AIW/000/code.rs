// Answer 0

#[test]
fn test_deref_mut() {
    struct BytesMut {
        data: Vec<u8>,
    }

    impl BytesMut {
        fn as_mut(&mut self) -> &mut [u8] {
            self.data.as_mut_slice()
        }

        fn deref_mut(&mut self) -> &mut [u8] {
            self.as_mut()
        }

        fn new() -> Self {
            BytesMut { data: Vec::new() }
        }
    }

    let mut bytes_mut = BytesMut::new();
    bytes_mut.data.extend_from_slice(&[1, 2, 3, 4]);

    let slice: &mut [u8] = bytes_mut.deref_mut();
    slice[0] = 10;

    assert_eq!(bytes_mut.data, vec![10, 2, 3, 4]);
}

#[test]
fn test_deref_mut_empty() {
    struct BytesMut {
        data: Vec<u8>,
    }

    impl BytesMut {
        fn as_mut(&mut self) -> &mut [u8] {
            self.data.as_mut_slice()
        }

        fn deref_mut(&mut self) -> &mut [u8] {
            self.as_mut()
        }

        fn new() -> Self {
            BytesMut { data: Vec::new() }
        }
    }

    let mut bytes_mut = BytesMut::new();
    let slice: &mut [u8] = bytes_mut.deref_mut();

    assert!(slice.is_empty());
}

