// Answer 0

#[test]
fn test_serialize_newtype_struct_valid() {
    use serde::Serialize;
    use serde_json::Value;
    use serde_json::ser::Serializer;

    struct ExampleStruct {
        field: String,
    }

    impl Serialize for ExampleStruct {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::ser::Serializer,
        {
            let mut state = serializer.serialize_struct("ExampleStruct", 1)?;
            state.serialize_field("field", &self.field)?;
            state.end()
        }
    }

    let example = ExampleStruct {
        field: String::from("test"),
    };
    
    let serializer = Serializer::new(Vec::new());
    let result = serializer.serialize_newtype_struct("ExampleStruct", &example).unwrap();
    let expected = serde_json::json!({
        "field": "test"
    });

    assert_eq!(result, expected);
}

#[test]
#[should_panic]
fn test_serialize_newtype_struct_invalid() {
    use serde::Serialize;
    use serde_json::ser::Serializer;

    struct InvalidStruct;

    let invalid_value = InvalidStruct;

    let serializer = Serializer::new(Vec::new());
    let _result = serializer.serialize_newtype_struct("InvalidStruct", &invalid_value).unwrap();
}

