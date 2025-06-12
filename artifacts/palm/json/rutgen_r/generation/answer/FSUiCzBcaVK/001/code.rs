// Answer 0

#[test]
#[should_panic]
fn test_to_string_pretty_invalid_serialize() {
    use serde::ser::Serialize;
    use serde_json::to_string_pretty;

    struct NonSerializable;

    // This struct does not implement Serialize
    let input = NonSerializable;

    // Attempt to serialize, should panic due to serialization failure
    let _ = to_string_pretty(&input).unwrap();
}

#[test]
#[should_panic]
fn test_to_string_pretty_non_string_map_keys() {
    use serde::Serialize;
    use serde_json::to_string_pretty;
    use std::collections::HashMap;

    struct NonStringKey;

    // Use a HashMap with a non-string key
    let mut map: HashMap<NonStringKey, i32> = HashMap::new();
    map.insert(NonStringKey, 10);

    // Attempt to serialize, should panic due to non-string map keys
    let _ = to_string_pretty(&map).unwrap();
}

