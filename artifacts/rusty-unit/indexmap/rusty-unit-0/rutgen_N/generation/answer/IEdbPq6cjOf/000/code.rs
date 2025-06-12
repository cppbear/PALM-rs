// Answer 0

#[test]
fn test_insert_replace_old_value() {
    use indexmap::IndexMap;

    let mut map = IndexMap::new();
    map.insert("key1", 1);
    let old_value = {
        let mut entry = map.entry("key1").or_insert(0);
        entry.insert(2)
    };
    
    assert_eq!(old_value, 1);
    assert_eq!(map["key1"], 2);
}

#[test]
fn test_insert_into_empty_entry() {
    use indexmap::IndexMap;

    let mut map = IndexMap::new();
    let old_value = {
        let mut entry = map.entry("key2").or_insert(0);
        entry.insert(3)
    };

    assert_eq!(old_value, 0);
    assert_eq!(map["key2"], 3);
}

#[test]
fn test_insert_with_same_value() {
    use indexmap::IndexMap;

    let mut map = IndexMap::new();
    map.insert("key3", 5);
    let old_value = {
        let mut entry = map.entry("key3").or_insert(0);
        entry.insert(5)
    };

    assert_eq!(old_value, 5);
    assert_eq!(map["key3"], 5);
}

