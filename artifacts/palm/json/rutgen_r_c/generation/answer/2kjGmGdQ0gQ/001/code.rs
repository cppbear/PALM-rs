// Answer 0

#[test]
fn test_to_string_with_non_serializable() {
    struct NonSerializable;

    impl Serialize for NonSerializable {
        fn serialize<S>(&self, _: S) -> Result<(), S::Error>
        where
            S: ser::Serializer,
        {
            Err(ser::Error::custom("Not serializable"))
        }
    }

    let value = NonSerializable;
    let result: Result<String> = to_string(&value);
    assert!(result.is_err());
}

#[test]
fn test_to_string_with_map_with_non_string_key() {
    use std::collections::HashMap;

    struct NonStringKey;

    impl Serialize for NonStringKey {
        fn serialize<S>(&self, _: S) -> Result<(), S::Error> where S: ser::Serializer {
            Ok(())
        }
    }

    let mut map = HashMap::new();
    map.insert(NonStringKey, "value");

    let result: Result<String> = to_string(&map);
    assert!(result.is_err());
}

#[test]
fn test_to_string_with_empty_struct() {
    #[derive(Serialize)]
    struct Empty;

    let value = Empty;
    let result: Result<String> = to_string(&value);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "{}"); // Expect an empty JSON object
}

