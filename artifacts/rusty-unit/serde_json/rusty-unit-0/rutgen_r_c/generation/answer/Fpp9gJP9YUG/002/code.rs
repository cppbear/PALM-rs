// Answer 0

#[test]
fn test_serialize_newtype_variant_success() {
    struct TestStruct {
        field: String,
    }
    
    impl Serialize for TestStruct {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer,
        {
            let mut state = serializer.serialize_struct("TestStruct", 1)?;
            state.serialize_field("field", &self.field)?;
            state.end()
        }
    }

    let test_value = TestStruct {
        field: "test_value".to_string(),
    };

    let serializer = Serializer;
    let result = serializer.serialize_newtype_variant("TestStruct", 0, "test_variant", &test_value);

    let expected = Ok(Value::Object(Map::from_iter(vec![
        (String::from("test_variant"), Value::String("test_value".to_string()))
    ])));

    assert_eq!(result, expected);
}

#[test]
#[should_panic]
fn test_serialize_newtype_variant_failure() {
    struct FailingStruct;

    impl Serialize for FailingStruct {
        fn serialize<S>(&self, _serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer,
        {
            panic!("This struct fails serialization");
        }
    }

    let failing_value = FailingStruct;

    let serializer = Serializer;
    let _result = serializer.serialize_newtype_variant("FailingStruct", 0, "fail_variant", &failing_value);
}

#[test]
fn test_serialize_newtype_variant_empty_variant() {
    struct EmptyStruct;

    impl Serialize for EmptyStruct {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer,
        {
            serializer.serialize_unit()
        }
    }

    let empty_value = EmptyStruct;
    
    let serializer = Serializer;
    let result = serializer.serialize_newtype_variant("EmptyStruct", 0, "empty_variant", &empty_value);

    let expected = Ok(Value::Object(Map::from_iter(vec![
        (String::from("empty_variant"), Value::Null)
    ])));

    assert_eq!(result, expected);
}

