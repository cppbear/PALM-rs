// Answer 0

#[test]
fn test_serialize_seq_with_some_length() {
    struct TestSerializer;

    impl TestSerializer {
        fn serialize_seq(self, len: Option<usize>) -> Result<SerializeVec, &'static str> {
            Ok(SerializeVec {
                vec: Vec::with_capacity(len.unwrap_or(0)),
            })
        }
    }

    struct SerializeVec {
        vec: Vec<u8>,
    }

    let serializer = TestSerializer;
    let result = serializer.serialize_seq(Some(10));

    assert!(result.is_ok());
    let serialize_vec = result.unwrap();
    assert_eq!(serialize_vec.vec.capacity(), 10);
}

#[test]
fn test_serialize_seq_with_none_length() {
    struct TestSerializer;

    impl TestSerializer {
        fn serialize_seq(self, len: Option<usize>) -> Result<SerializeVec, &'static str> {
            Ok(SerializeVec {
                vec: Vec::with_capacity(len.unwrap_or(0)),
            })
        }
    }

    struct SerializeVec {
        vec: Vec<u8>,
    }

    let serializer = TestSerializer;
    let result = serializer.serialize_seq(None);

    assert!(result.is_ok());
    let serialize_vec = result.unwrap();
    assert_eq!(serialize_vec.vec.capacity(), 0);
}

#[test]
fn test_serialize_seq_with_zero_length() {
    struct TestSerializer;

    impl TestSerializer {
        fn serialize_seq(self, len: Option<usize>) -> Result<SerializeVec, &'static str> {
            Ok(SerializeVec {
                vec: Vec::with_capacity(len.unwrap_or(0)),
            })
        }
    }

    struct SerializeVec {
        vec: Vec<u8>,
    }

    let serializer = TestSerializer;
    let result = serializer.serialize_seq(Some(0));

    assert!(result.is_ok());
    let serialize_vec = result.unwrap();
    assert_eq!(serialize_vec.vec.capacity(), 0);
}

