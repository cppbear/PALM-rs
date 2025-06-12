// Answer 0

#[test]
fn test_try_into_mut_non_unique() {
    struct NonUniqueBytes {
        data: Vec<u8>,
    }

    impl NonUniqueBytes {
        fn new(data: Vec<u8>) -> Self {
            NonUniqueBytes { data }
        }

        fn is_unique(&self) -> bool {
            false
        }
        
        fn try_into_mut(self) -> Result<BytesMut, NonUniqueBytes> {
            if self.is_unique() {
                Ok(BytesMut::from(self.data.clone()))
            } else {
                Err(self)
            }
        }
    }

    let non_unique_bytes = NonUniqueBytes::new(b"hello".to_vec());
    let result = non_unique_bytes.try_into_mut();
    
    assert!(result.is_err());
    assert_eq!(result.err().unwrap().data, b"hello".to_vec());
}

