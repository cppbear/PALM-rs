// Answer 0

#[test]
#[should_panic]
fn test_to_vec_non_string_map_keys() {
    use serde::Serialize;
    use std::collections::HashMap;

    #[derive(Serialize)]
    struct NonStringKey {
        key: i32,
        value: String,
    }

    let mut map: HashMap<NonStringKey, String> = HashMap::new();
    map.insert(NonStringKey { key: 1, value: "value".to_string() }, "test".to_string());

    let result: Result<Vec<u8>, _> = to_vec(&map);
    assert!(result.is_err());
}

#[test]
fn test_to_vec_serialize_fail() {
    use serde::Serialize;

    struct FailSerialize;

    impl Serialize for FailSerialize {
        fn serialize<S>(&self, _: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer,
        {
            Err(serde::ser::Error::custom("serialization failed"))
        }
    }

    let value = FailSerialize;
    let result: Result<Vec<u8>, _> = to_vec(&value);
    assert!(result.is_err());
}

