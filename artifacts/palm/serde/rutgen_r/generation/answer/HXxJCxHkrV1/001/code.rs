// Answer 0

#[test]
fn test_end_returns_err_on_serialize_value_error() {
    struct MockMap;

    impl MockMap {
        fn serialize_value(&self, _: &Content) -> Result<(), &'static str> {
            Err("serialization error")
        }
    }

    struct TestStruct {
        map: MockMap,
        fields: Vec<i32>,
    }

    impl TestStruct {
        fn end(self) -> Result<(), &'static str> {
            self.map.serialize_value(&Content::Seq(self.fields)).map_err(|e| e)
        }
    }

    let test_struct = TestStruct {
        map: MockMap,
        fields: vec![1, 2, 3],
    };

    let result = test_struct.end();
    assert!(result.is_err());
    assert_eq!(result.err(), Some("serialization error"));
}

