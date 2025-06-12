// Answer 0

#[test]
fn test_serialize_bytes_should_return_err() {
    struct TestStruct;

    impl TestStruct {
        fn serialize_bytes(self, value: &[u8]) -> Result<(), String> {
            Err(key_must_be_a_string())
        }
    }

    fn key_must_be_a_string() -> String {
        "Key must be a string".to_string()
    }

    let test_struct = TestStruct;

    let result = test_struct.serialize_bytes(&[1, 2, 3, 4]);

    assert_eq!(result, Err("Key must be a string".to_string()));
}

