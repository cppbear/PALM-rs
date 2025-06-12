// Answer 0

#[test]
fn test_serialize_seq_invalid_type() {
    struct TestSerializer;

    impl TestSerializer {
        fn bad_type(&self, _: Unsupported) -> Result<(), String> {
            Err("Unsupported sequence type".to_string())
        }

        fn serialize_seq(self, _: Option<usize>) -> Result<(), String> {
            Err(self.bad_type(Unsupported::Sequence))
        }
    }

    enum Unsupported {
        Sequence,
    }

    let serializer = TestSerializer;
    let result = serializer.serialize_seq(None);
    assert_eq!(result, Err("Unsupported sequence type".to_string()));
}

#[test]
fn test_serialize_seq_with_some_option() {
    struct TestSerializer;

    impl TestSerializer {
        fn bad_type(&self, _: Unsupported) -> Result<(), String> {
            Err("Unsupported sequence type".to_string())
        }

        fn serialize_seq(self, _: Option<usize>) -> Result<(), String> {
            Err(self.bad_type(Unsupported::Sequence))
        }
    }

    enum Unsupported {
        Sequence,
    }

    let serializer = TestSerializer;
    let result = serializer.serialize_seq(Some(5));
    assert_eq!(result, Err("Unsupported sequence type".to_string()));
}

