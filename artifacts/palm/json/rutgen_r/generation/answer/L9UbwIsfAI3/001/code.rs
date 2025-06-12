// Answer 0

#[test]
fn test_serialize_unit() {
    use serde_json::ser::Serializer;
    use serde_json::Error;

    struct TestStruct;

    impl TestStruct {
        fn serialize(&self) -> Result<(), Error> {
            self.serialize_unit()
        }

        fn serialize_unit(self) -> Result<(), Error> {
            Err(self.key_must_be_a_string())
        }

        fn key_must_be_a_string(&self) -> Error {
            serde_json::Error::custom("key must be a string")
        }
    }

    let test_instance = TestStruct;
    let result = test_instance.serialize();
    assert!(result.is_err());
    assert_eq!(result.unwrap_err().to_string(), "key must be a string");
}

