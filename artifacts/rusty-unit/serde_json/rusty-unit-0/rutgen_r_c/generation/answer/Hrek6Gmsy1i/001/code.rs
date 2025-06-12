// Answer 0

#[test]
fn test_remove_entry_with_order_preserved() {
    use crate::Value;
    #[cfg(feature = "preserve_order")]
    use indexmap::IndexMap;

    let mut map: IndexMap<String, Value> = IndexMap::new();
    map.insert("key1".to_string(), Value::Number(serde_json::Number::from(1)));
    map.insert("key2".to_string(), Value::Number(serde_json::Number::from(2)));

    let entry = map.get_entry("key1").unwrap();

    let (key, value) = entry.remove_entry();

    assert_eq!(key, "key1");
    assert_eq!(value, Value::Number(serde_json::Number::from(1)));
    assert!(!map.contains_key("key1"));
}

#[test]
fn test_remove_entry_without_order_preserved() {
    use crate::Value;
    #[cfg(not(feature = "preserve_order"))]
    use std::collections::BTreeMap;

    let mut map: BTreeMap<String, Value> = BTreeMap::new();
    map.insert("key1".to_string(), Value::Number(serde_json::Number::from(1)));
    map.insert("key2".to_string(), Value::Number(serde_json::Number::from(2)));

    let entry: crate::OccupiedEntry<'_> = {
        let occupied = map.get_mut("key1").unwrap();
        crate::OccupiedEntry { occupied }
    };

    let (key, value) = entry.remove_entry();

    assert_eq!(key, "key1");
    assert_eq!(value, Value::Number(serde_json::Number::from(1)));
    assert!(!map.contains_key("key1"));
}

#[test]
#[should_panic]
fn test_remove_entry_nonexistent_key() {
    use crate::Value;
    let mut map: std::collections::BTreeMap<String, Value> = std::collections::BTreeMap::new();

    let entry: crate::OccupiedEntry<'_> = {
        let occupied = map.get_mut("nonexistent").unwrap(); // This should panic
        crate::OccupiedEntry { occupied }
    };
    
    let _ = entry.remove_entry();
}

