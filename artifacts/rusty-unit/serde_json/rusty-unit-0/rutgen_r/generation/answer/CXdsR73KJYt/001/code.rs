// Answer 0

#[test]
fn test_serialize_struct_should_return_err() {
    struct TestSerializer;

    impl TestSerializer {
        fn serialize_struct(self, _name: &'static str, _len: usize) -> Result<(), String> {
            Err(self.key_must_be_a_string())
        }

        fn key_must_be_a_string(&self) -> String {
            "key must be a string".to_string()
        }
    }

    let serializer = TestSerializer;
    let result = serializer.serialize_struct("TestStruct", 0);
    assert_eq!(result, Err("key must be a string".to_string()));
}

#[test]
fn test_serialize_struct_with_non_empty_len_should_return_err() {
    struct TestSerializer;

    impl TestSerializer {
        fn serialize_struct(self, _name: &'static str, _len: usize) -> Result<(), String> {
            Err(self.key_must_be_a_string())
        }

        fn key_must_be_a_string(&self) -> String {
            "key must be a string".to_string()
        }
    }

    let serializer = TestSerializer;
    let result = serializer.serialize_struct("AnotherStruct", 5);
    assert_eq!(result, Err("key must be a string".to_string()));
}

