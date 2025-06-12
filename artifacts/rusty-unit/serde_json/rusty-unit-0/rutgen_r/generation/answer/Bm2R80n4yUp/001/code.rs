// Answer 0

#[test]
fn test_get_existing_key() {
    use std::collections::HashMap;
    use std::hash::Hash;
    use serde_json::Value;

    let mut map = HashMap::new();
    map.insert("key1".to_string(), Value::String("value1".to_string()));
    map.insert("key2".to_string(), Value::String("value2".to_string()));

    let result = map.get("key1");
    assert_eq!(result, Some(&Value::String("value1".to_string())));
}

#[test]
fn test_get_non_existing_key() {
    use std::collections::HashMap;
    use std::hash::Hash;
    use serde_json::Value;

    let mut map = HashMap::new();
    map.insert("key1".to_string(), Value::String("value1".to_string()));

    let result = map.get("key2");
    assert_eq!(result, None);
}

#[test]
fn test_get_with_different_borrowed_form() {
    use std::collections::HashMap;
    use std::hash::Hash;
    use serde_json::Value;

    let mut map = HashMap::new();
    map.insert("key1".to_string(), Value::String("value1".to_string()));

    let borrowed_key: &str = "key1";
    let result = map.get(borrowed_key);
    assert_eq!(result, Some(&Value::String("value1".to_string())));
}

#[test]
fn test_get_with_nil_key() {
    use std::collections::HashMap;
    use std::hash::Hash;
    use serde_json::Value;

    let map: HashMap<String, Value> = HashMap::new();
    let result = map.get("");
    assert_eq!(result, None);
}

#[test]
#[should_panic]
fn test_get_panic_on_wrong_type() {
    use std::collections::HashMap;
    use std::hash::Hash;
    use serde_json::Value;

    let mut map = HashMap::new();
    map.insert("key1".to_string(), Value::String("value1".to_string()));

    // This will not work since get expects a borrowed form of the key type
    let _result = map.get(&42); // This should panic
}

