// Answer 0

#[test]
fn test_serialize_newtype_variant_with_non_string_key() {
    use serde::ser::Serialize;
    use serde_json::Error;

    struct NonStringStruct;

    impl Serialize for NonStringStruct {
        fn serialize<S>(&self, _serializer: S) -> Result<S::Ok, S::Error> 
        where
            S: serde::Serializer,
        {
            Ok(())
        }
    }

    let non_string_value = NonStringStruct;

    let result: Result<String, Error> = serialize_newtype_variant("Example", 0, "ExampleVariant", &non_string_value);
    
    assert!(result.is_err());
    assert_eq!(result.unwrap_err().to_string(), key_must_be_a_string().to_string());
}

