// Answer 0

#[test]
fn test_serialize_seq_with_none_length() {
    struct TestSerializer;

    impl TestSerializer {
        fn serialize_seq(self, len: Option<usize>) -> Result<SerializeVec> {
            Ok(SerializeVec {
                vec: Vec::with_capacity(len.unwrap_or(0)),
            })
        }
    }

    struct SerializeVec {
        vec: Vec<u8>,
    }

    let serializer = TestSerializer;
    let result = serializer.serialize_seq(None).unwrap();
    assert_eq!(result.vec.capacity(), 0);
}

#[test]
fn test_serialize_seq_with_some_length() {
    struct TestSerializer;

    impl TestSerializer {
        fn serialize_seq(self, len: Option<usize>) -> Result<SerializeVec> {
            Ok(SerializeVec {
                vec: Vec::with_capacity(len.unwrap_or(0)),
            })
        }
    }

    struct SerializeVec {
        vec: Vec<u8>,
    }

    let serializer = TestSerializer;
    let result = serializer.serialize_seq(Some(10)).unwrap();
    assert_eq!(result.vec.capacity(), 10);
}

