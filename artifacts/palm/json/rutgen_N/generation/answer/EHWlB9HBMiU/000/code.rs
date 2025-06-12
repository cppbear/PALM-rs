// Answer 0

#[test]
fn test_retain_with_true_predicate() {
    use serde_json::{Value, Map};

    let mut map: Map<String, Value> = Map::new();
    map.insert("key1".to_string(), Value::from(1));
    map.insert("key2".to_string(), Value::from(2));
    map.insert("key3".to_string(), Value::from(3));

    map.retain(|_k, _v| true); // predicate returns true for all elements

    assert_eq!(map.len(), 3);
}

#[test]
fn test_retain_with_false_predicate() {
    use serde_json::{Value, Map};

    let mut map: Map<String, Value> = Map::new();
    map.insert("key1".to_string(), Value::from(1));
    map.insert("key2".to_string(), Value::from(2));
    map.insert("key3".to_string(), Value::from(3));

    map.retain(|_k, _v| false); // predicate returns false for all elements

    assert_eq!(map.len(), 0);
}

#[test]
fn test_retain_with_conditional_predicate() {
    use serde_json::{Value, Map};

    let mut map: Map<String, Value> = Map::new();
    map.insert("key1".to_string(), Value::from(1));
    map.insert("key2".to_string(), Value::from(2));
    map.insert("key3".to_string(), Value::from(3));

    map.retain(|k, v| {
        if k == "key2" && v.is_number() {
            *v = Value::from(20); // Mutate the value as part of the condition
            true
        } else {
            false
        }
    });

    assert_eq!(map.len(), 1);
    assert_eq!(map.get("key2").unwrap(), &Value::from(20));
}

#[test]
fn test_retain_with_empty_map() {
    use serde_json::{Value, Map};

    let mut map: Map<String, Value> = Map::new();

    map.retain(|_k, _v| true); // Retain with a predicate

    assert_eq!(map.len(), 0); // Should still be empty
}

