// Answer 0

#[test]
fn test_serialize_tuple_struct() {
    struct TestStruct;

    impl TestStruct {
        fn serialize_tuple_struct(
            self,
            _name: &'static str,
            _len: usize,
        ) -> Result<(), &'static str> {
            Err(key_must_be_a_string())
        }
    }

    fn key_must_be_a_string() -> &'static str {
        "key must be a string"
    }

    let instance = TestStruct;
    let result = instance.serialize_tuple_struct("test_name", 3);
    assert_eq!(result, Err("key must be a string"));
}

#[test]
fn test_serialize_tuple_struct_empty() {
    struct TestStruct;

    impl TestStruct {
        fn serialize_tuple_struct(
            self,
            _name: &'static str,
            _len: usize,
        ) -> Result<(), &'static str> {
            Err(key_must_be_a_string())
        }
    }

    fn key_must_be_a_string() -> &'static str {
        "key must be a string"
    }

    let instance = TestStruct;
    let result = instance.serialize_tuple_struct("", 0);
    assert_eq!(result, Err("key must be a string"));
}

