// Answer 0

#[test]
fn test_to_vec_pretty_with_non_serializable_keys() {
    use serde::Serialize;
    use serde_json::Error;

    struct NonSerializableKey {
        id: i32,
    }

    impl Serialize for NonSerializableKey {
        fn serialize<S>(&self, _: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer,
        {
            Err(serde::ser::Error::custom("non-serializable key"))
        }
    }

    let map_with_non_serializable_key = std::collections::HashMap::<NonSerializableKey, String>::new();

    let result: Result<Vec<u8>, Error> = serde_json::to_vec_pretty(&map_with_non_serializable_key);

    assert!(result.is_err());
}

#[test]
fn test_to_vec_pretty_with_failing_serialization() {
    use serde::Serialize;
    use serde_json::Error;

    struct StructThatFailsToSerialize;

    impl Serialize for StructThatFailsToSerialize {
        fn serialize<S>(&self, _: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer,
        {
            Err(serde::ser::Error::custom("fake serialization error"))
        }
    }

    let value = StructThatFailsToSerialize;

    let result: Result<Vec<u8>, Error> = serde_json::to_vec_pretty(&value);

    assert!(result.is_err());
}

