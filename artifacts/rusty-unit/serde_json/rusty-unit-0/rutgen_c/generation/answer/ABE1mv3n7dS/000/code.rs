// Answer 0

#[test]
fn test_remove_occupied_entry_preserve_order() {
    #[cfg(feature = "preserve_order")]
    struct TestOccupiedEntry<'a> {
        occupied: OccupiedEntryImpl<'a>,
    }

    let mut map: MapImpl<String, Value> = MapImpl::new();
    map.insert("key1".to_owned(), Value::Number(serde_json::Number::from(1)));
    map.insert("key2".to_owned(), Value::Number(serde_json::Number::from(2)));

    let mut occupied_entry = TestOccupiedEntry {
        occupied: map.entry("key1").or_insert(Value::Null),
    };

    let value = occupied_entry.occupied.remove();
    assert_eq!(value, Value::Number(serde_json::Number::from(1)));
    assert!(!map.contains_key("key1"));
}

#[test]
fn test_remove_occupied_entry_no_preserve_order() {
    #[cfg(not(feature = "preserve_order"))]
    struct TestOccupiedEntry<'a> {
        occupied: OccupiedEntryImpl<'a>,
    }

    let mut map: MapImpl<String, Value> = MapImpl::new();
    map.insert("key1".to_owned(), Value::Number(serde_json::Number::from(1)));
    map.insert("key2".to_owned(), Value::Number(serde_json::Number::from(2)));

    let mut occupied_entry = TestOccupiedEntry {
        occupied: map.entry("key1").or_insert(Value::Null),
    };

    let value = occupied_entry.occupied.remove();
    assert_eq!(value, Value::Number(serde_json::Number::from(1)));
    assert!(!map.contains_key("key1"));
}

