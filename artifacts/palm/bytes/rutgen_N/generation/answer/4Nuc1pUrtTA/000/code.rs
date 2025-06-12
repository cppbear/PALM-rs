// Answer 0

#[test]
fn test_try_into_mut_unique() {
    struct Bytes {
        unique: bool,
        data: Vec<u8>,
    }

    impl Bytes {
        fn from(data: Vec<u8>) -> Self {
            Bytes { unique: true, data }
        }

        fn is_unique(&self) -> bool {
            self.unique
        }

        fn into(self) -> BytesMut {
            BytesMut::from(self.data)
        }

        fn try_into_mut(self) -> Result<BytesMut, Bytes> {
            if self.is_unique() {
                Ok(self.into())
            } else {
                Err(self)
            }
        }
    }

    struct BytesMut {
        data: Vec<u8>,
    }

    impl From<Vec<u8>> for BytesMut {
        fn from(vec: Vec<u8>) -> Self {
            BytesMut { data: vec }
        }
    }

    let bytes = Bytes::from(b"hello".to_vec());
    assert_eq!(bytes.try_into_mut(), Ok(BytesMut::from(b"hello".to_vec())));
}

#[test]
fn test_try_into_mut_non_unique() {
    struct Bytes {
        unique: bool,
        data: Vec<u8>,
    }

    impl Bytes {
        fn from(data: Vec<u8>, unique: bool) -> Self {
            Bytes { unique, data }
        }

        fn is_unique(&self) -> bool {
            self.unique
        }

        fn into(self) -> BytesMut {
            BytesMut::from(self.data)
        }

        fn try_into_mut(self) -> Result<BytesMut, Bytes> {
            if self.is_unique() {
                Ok(self.into())
            } else {
                Err(self)
            }
        }
    }

    struct BytesMut {
        data: Vec<u8>,
    }

    impl From<Vec<u8>> for BytesMut {
        fn from(vec: Vec<u8>) -> Self {
            BytesMut { data: vec }
        }
    }

    let bytes = Bytes::from(b"hello".to_vec(), false);
    assert_eq!(bytes.try_into_mut(), Err(bytes));
}

