// Answer 0

#[test]
fn test_serialize_seq_err() {
    struct TestSerializer;

    impl TestSerializer {
        fn serialize_seq(self, _len: Option<usize>) -> Result<(), String> {
            Err("key must be a string".to_string())
        }
    }

    let serializer = TestSerializer;
    let result = serializer.serialize_seq(Some(10));
    assert_eq!(result, Err("key must be a string".to_string()));
}

#[test]
fn test_serialize_seq_err_with_none_len() {
    struct TestSerializer;

    impl TestSerializer {
        fn serialize_seq(self, _len: Option<usize>) -> Result<(), String> {
            Err("key must be a string".to_string())
        }
    }

    let serializer = TestSerializer;
    let result = serializer.serialize_seq(None);
    assert_eq!(result, Err("key must be a string".to_string()));
}

