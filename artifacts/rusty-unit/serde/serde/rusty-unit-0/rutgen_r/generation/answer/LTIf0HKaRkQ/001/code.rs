// Answer 0

#[test]
fn test_end_serialization_error() {
    struct MockMap;

    impl MockMap {
        fn serialize_value(&self, _: &Content) -> Result<(), String> {
            Err("Serialization Error".to_string())
        }
        
        fn end(&self) -> Result<(), String> {
            Ok(())
        }
    }

    struct TestSerializer {
        map: MockMap,
        name: String,
        fields: Vec<String>,
    }

    impl TestSerializer {
        fn end(mut self) -> Result<(), String> {
            match self.map.serialize_value(&Content::TupleStruct(self.name, self.fields)) {
                Ok(_) => self.map.end(),
                Err(err) => Err(err),
            }
        }
    }

    let serializer = TestSerializer {
        map: MockMap,
        name: "TestStruct".to_string(),
        fields: vec!["field1".to_string(), "field2".to_string()],
    };

    let result = serializer.end();
    assert_eq!(result, Err("Serialization Error".to_string()));
}

