// Answer 0

#[test]
#[should_panic]
fn test_to_string_pretty_with_non_serializable_key() {
    use serde::Serialize;
    use std::collections::HashMap;

    struct NonSerializable;

    impl Serialize for NonSerializable {
        // A dummy implementation that does not satisfy serialization
        fn serialize<S>(&self, _: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer,
        {
            Err(serde::ser::Error::custom("failed to serialize"))
        }
    }

    let mut map: HashMap<NonSerializable, String> = HashMap::new();
    map.insert(NonSerializable, "value".to_string());

    // This should panic because we're trying to serialize a map with non-serializable keys
    let result = serde_json::to_string_pretty(&map);
    assert!(result.is_err());
}

#[test]
fn test_to_string_pretty_with_failed_serialization() {
    use serde::Serialize;
    use serde_json::Result;

    struct FailsOnSerialize;

    impl Serialize for FailsOnSerialize {
        // A dummy implementation that fails serialization
        fn serialize<S>(&self, _: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer,
        {
            Err(serde::ser::Error::custom("serialization error"))
        }
    }

    let instance = FailsOnSerialize;

    let result = serde_json::to_string_pretty(&instance);
    assert!(result.is_err());
}

