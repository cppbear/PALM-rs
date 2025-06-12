// Answer 0

#[test]
fn test_sort_keys() {
    #[cfg(feature = "preserve_order")]
    struct TestMap {
        map: std::collections::BTreeMap<String, i32>,
    }

    #[cfg(feature = "preserve_order")]
    impl TestMap {
        fn new(entries: Vec<(String, i32)>) -> Self {
            let map = entries.into_iter().collect();
            TestMap { map }
        }

        fn sort_keys(&mut self) {
            self.map.sort_keys();
        }
    }

    #[cfg(feature = "preserve_order")]
    let mut test_map = TestMap::new(vec![
        ("b".to_string(), 2),
        ("a".to_string(), 1),
        ("c".to_string(), 3)
    ]);
    #[cfg(feature = "preserve_order")]
    test_map.sort_keys();
    
    #[cfg(feature = "preserve_order")]
    let sorted_keys: Vec<String> = test_map.map.keys().cloned().collect();
    #[cfg(feature = "preserve_order")]
    assert_eq!(sorted_keys, vec!["a".to_string(), "b".to_string(), "c".to_string()]);
}

#[test]
#[should_panic]
fn test_sort_keys_should_panic_if_feature_not_enabled() {
    struct TestMap {
        map: std::collections::BTreeMap<String, i32>,
    }

    impl TestMap {
        fn new(entries: Vec<(String, i32)>) -> Self {
            let map = entries.into_iter().collect();
            TestMap { map }
        }

        fn sort_keys(&mut self) {
            // This line would panic or do nothing if "preserve_order" isn't enabled
            self.map.sort_keys();
        }
    }

    let mut test_map = TestMap::new(vec![
        ("b".to_string(), 2),
        ("a".to_string(), 1),
        ("c".to_string(), 3)
    ]);
    test_map.sort_keys();
}

