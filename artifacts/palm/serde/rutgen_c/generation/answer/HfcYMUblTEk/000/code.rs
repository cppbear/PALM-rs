// Answer 0

#[test]
fn test_end_success() {
    struct MockMap {
        serialized: Option<Content>,
    }

    impl ser::SerializeMap for MockMap {
        type Ok = ();
        type Error = Error;

        fn serialize_value(&mut self, value: &Content) -> Result<(), Self::Error> {
            self.serialized = Some(value.clone());
            Ok(())
        }

        fn end(self) -> Result<Self::Ok, Self::Error> {
            Ok(())
        }
    }

    let mut map = MockMap { serialized: None };
    let name = "test_struct";
    let fields = vec![
        ("field1", Content::U32(42)),
        ("field2", Content::String("value".to_string())),
    ];

    let serialize_struct = SerializeStructVariantAsMapValue {
        map,
        name,
        fields,
    };

    let result = serialize_struct.end();
    assert!(result.is_ok());
    assert!(matches!(serialize_struct.map.serialized, Some(Content::Struct("test_struct", _))));
}

#[test]
fn test_end_with_error() {
    struct ErroneousMap;

    impl ser::SerializeMap for ErroneousMap {
        type Ok = ();
        type Error = Error;

        fn serialize_value(&mut self, _value: &Content) -> Result<(), Self::Error> {
            Err(Error)
        }

        fn end(self) -> Result<Self::Ok, Self::Error> {
            Ok(())
        }
    }

    let mut map = ErroneousMap;
    let name = "test_struct";
    let fields = vec![
        ("field1", Content::U32(42)),
        ("field2", Content::String("value".to_string())),
    ];

    let serialize_struct = SerializeStructVariantAsMapValue {
        map,
        name,
        fields,
    };

    let result = serialize_struct.end();
    assert!(result.is_err());
}

