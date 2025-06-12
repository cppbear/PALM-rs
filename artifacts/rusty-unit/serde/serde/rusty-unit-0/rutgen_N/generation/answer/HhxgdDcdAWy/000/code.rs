// Answer 0

#[test]
fn test_end_success() {
    struct MockMap;
    struct MockError;

    impl MockMap {
        fn serialize_value(&self, _: &Content) -> Result<(), MockError> {
            Ok(())
        }
    }

    struct TestStruct {
        map: MockMap,
        name: String,
        fields: Vec<String>,
    }

    impl TestStruct {
        fn end(self) -> Result<(), MockError> {
            self.map.serialize_value(&Content::Struct(self.name, self.fields))
        }
    }

    let test_struct = TestStruct {
        map: MockMap,
        name: "test".to_string(),
        fields: vec!["field1".to_string(), "field2".to_string()],
    };

    assert!(test_struct.end().is_ok());
}

#[test]
#[should_panic]
fn test_end_failure() {
    struct MockMap;
    struct MockError;

    impl MockMap {
        fn serialize_value(&self, _: &Content) -> Result<(), MockError> {
            Err(MockError)
        }
    }

    struct TestStruct {
        map: MockMap,
        name: String,
        fields: Vec<String>,
    }

    impl TestStruct {
        fn end(self) -> Result<(), MockError> {
            self.map.serialize_value(&Content::Struct(self.name, self.fields))
        }
    }

    let test_struct = TestStruct {
        map: MockMap,
        name: "test".to_string(),
        fields: vec!["field1".to_string(), "field2".to_string()],
    };

    let _ = test_struct.end().unwrap(); // This will panic due to unwrap on Err
}

