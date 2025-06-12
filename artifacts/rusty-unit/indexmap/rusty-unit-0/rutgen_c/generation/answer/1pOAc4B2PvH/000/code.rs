// Answer 0

#[test]
fn test_retain_in_order() {
    struct TestEntry {
        key: usize,
        value: String,
    }

    let mut index_map = IndexMapCore::new();
    index_map.entries.push(Bucket { hash: HashValue::default(), key: 1, value: "one".to_string() });
    index_map.entries.push(Bucket { hash: HashValue::default(), key: 2, value: "two".to_string() });
    index_map.entries.push(Bucket { hash: HashValue::default(), key: 3, value: "three".to_string() });
    
    index_map.indices = Indices::with_capacity(3);
    index_map.indices.insert(1, 0);
    index_map.indices.insert(2, 1);
    index_map.indices.insert(3, 2);

    index_map.retain_in_order(|key, value| {
        if *key % 2 == 0 {
            *value = format!("modified {}", value);
            true
        } else {
            false
        }
    });

    assert_eq!(index_map.entries.len(), 1);
    assert_eq!(index_map.entries[0].key, 2);
    assert_eq!(index_map.entries[0].value, "modified two");
}

#[test]
fn test_retain_in_order_with_no_entries() {
    let mut index_map: IndexMapCore<usize, String> = IndexMapCore::new();
    index_map.retain_in_order(|_, _| true);
    assert_eq!(index_map.entries.len(), 0);
}

#[test]
fn test_retain_in_order_with_all_removal() {
    let mut index_map = IndexMapCore::new();
    index_map.entries.push(Bucket { hash: HashValue::default(), key: 1, value: "one".to_string() });
    index_map.entries.push(Bucket { hash: HashValue::default(), key: 2, value: "two".to_string() });

    index_map.indices = Indices::with_capacity(2);
    index_map.indices.insert(1, 0);
    index_map.indices.insert(2, 1);

    index_map.retain_in_order(|_, _| false);
    assert_eq!(index_map.entries.len(), 0);
    assert_eq!(index_map.indices.len(), 0);
}

