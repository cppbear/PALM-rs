// Answer 0

#[test]
fn test_sort_keys_with_preserve_order() {
    #[cfg(feature = "preserve_order")]
    {
        use indexmap::IndexMap;

        let mut map = Map::with_capacity(3);
        map.insert("c".to_string(), Value::String("value_c".to_string()));
        map.insert("a".to_string(), Value::String("value_a".to_string()));
        map.insert("b".to_string(), Value::String("value_b".to_string()));

        map.sort_keys();

        let keys: Vec<_> = map.keys().cloned().collect();
        assert_eq!(keys, vec!["a", "b", "c"]);
    }
}

#[test]
fn test_sort_keys_without_preserve_order() {
    #[cfg(not(feature = "preserve_order"))]
    {
        let mut map = Map::with_capacity(3);
        map.insert("c".to_string(), Value::String("value_c".to_string()));
        map.insert("a".to_string(), Value::String("value_a".to_string()));
        map.insert("b".to_string(), Value::String("value_b".to_string()));

        // Sorting keys should not change the order in a BTreeMap
        map.sort_keys();

        let keys: Vec<_> = map.keys().cloned().collect();
        assert_eq!(keys, vec!["a", "b", "c"]); // Should remain in sorted order
    }
}

