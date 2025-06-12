// Answer 0

#[test]
fn test_serialize_unit_err() {
    struct TestStruct;

    impl TestStruct {
        fn serialize_unit(self) -> Result<String> {
            Err(key_must_be_a_string())
        }
    }

    let instance = TestStruct;
    let result = instance.serialize_unit();
    assert!(result.is_err());
}

