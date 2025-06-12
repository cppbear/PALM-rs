// Answer 0

#[test]
fn test_deref() {
    struct BytesMut {
        data: Vec<u8>,
    }

    impl BytesMut {
        fn as_ref(&self) -> &[u8] {
            &self.data
        }

        fn new(data: Vec<u8>) -> Self {
            Self { data }
        }
        
        fn deref(&self) -> &[u8] {
            self.as_ref()
        }
    }

    let bytes_mut = BytesMut::new(vec![1, 2, 3, 4, 5]);
    assert_eq!(bytes_mut.deref(), &[1, 2, 3, 4, 5]);
}

#[test]
fn test_empty_deref() {
    struct BytesMut {
        data: Vec<u8>,
    }

    impl BytesMut {
        fn as_ref(&self) -> &[u8] {
            &self.data
        }

        fn new(data: Vec<u8>) -> Self {
            Self { data }
        }
        
        fn deref(&self) -> &[u8] {
            self.as_ref()
        }
    }

    let bytes_mut = BytesMut::new(vec![]);
    assert_eq!(bytes_mut.deref(), &[]);
}

