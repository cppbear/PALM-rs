// Answer 0

#[test]
fn test_try_into_mut_success_unique() {
    use bytes::{Bytes, BytesMut};

    struct UniqueBytes {
        bytes: Bytes,
    }

    impl UniqueBytes {
        fn new(data: Vec<u8>) -> Self {
            let bytes = Bytes::from(data);
            UniqueBytes { bytes }
        }

        fn try_into_mut(self) -> Result<BytesMut, Bytes> {
            if self.bytes.is_unique() {
                Ok(self.bytes.into())
            } else {
                Err(self.bytes)
            }
        }
    }

    let unique_data = b"unique".to_vec();
    let unique_bytes = UniqueBytes::new(unique_data);
    let result = unique_bytes.try_into_mut();
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), BytesMut::from(&b"unique"[..]));
}

#[test]
#[should_panic]
fn test_try_into_mut_fail_not_unique() {
    use bytes::{Bytes, BytesMut};

    struct NonUniqueBytes {
        bytes: Bytes,
    }

    impl NonUniqueBytes {
        fn new(data: Vec<u8>) -> Self {
            let bytes = Bytes::from(data);
            NonUniqueBytes { bytes }
        }

        fn try_into_mut(self) -> Result<BytesMut, Bytes> {
            if self.bytes.is_unique() {
                Ok(self.bytes.into())
            } else {
                Err(self.bytes)
            }
        }
    }

    let non_unique_data = b"non-unique".to_vec();
    let non_unique_bytes = NonUniqueBytes::new(non_unique_data);
    let _result = non_unique_bytes.try_into_mut(); // This will not panic but return Err
}

