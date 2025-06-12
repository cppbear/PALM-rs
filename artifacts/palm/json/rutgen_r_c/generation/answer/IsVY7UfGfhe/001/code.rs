// Answer 0

#[test]
fn test_to_vec_with_non_serializable_key() {
    use serde::ser::Serializer;
    use serde::Serialize;

    struct NonSerializable;

    impl Serialize for NonSerializable {
        fn serialize<S>(&self, _: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            Err(ser::Error::custom("Serialization error"))
        }
    }

    let value = vec![
        (NonSerializable, "value")
    ];

    let result = to_vec(&value);
    assert!(result.is_err());
}

#[test]
fn test_to_vec_with_non_string_key() {
    use serde::ser::Serializer;
    use serde::Serialize;
    use std::collections::HashMap;

    struct CustomKey;

    impl Serialize for CustomKey {
        fn serialize<S>(&self, _: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            Err(ser::Error::custom("Key must be a string"))
        }
    }

    let mut map = HashMap::new();
    map.insert(CustomKey, "value");

    let result = to_vec(&map);
    assert!(result.is_err());
}

