// Answer 0

#[test]
fn test_append_new_value_to_existing_entry() {
    #[derive(Debug)]
    struct TestHeaderMap {
        map: HeaderMap,
    }

    impl TestHeaderMap {
        fn new() -> Self {
            let map = HeaderMap {
                mask: 1,
                indices: Box::new([]),
                entries: vec![
                    Bucket {
                        hash: 0, // Example hash
                        key: "host".parse().unwrap(),
                        value: "world".to_string(),
                        links: None,
                    },
                ],
                extra_values: vec![],
                danger: Danger::default(),
            };
            TestHeaderMap { map }
        }

        fn entry_mut(&mut self, key: &str) -> Option<&mut OccupiedEntry<'_, String>> {
            if self.map.entries.len() > 0 && self.map.entries[0].key == key.parse().unwrap() {
                Some(OccupiedEntry {
                    map: &mut self.map,
                    probe: 0,
                    index: 0,
                })
            } else {
                None
            }
        }
    }

    let mut test_map = TestHeaderMap::new();
    if let Some(mut occupied) = test_map.entry_mut("host") {
        occupied.append("earth".to_string());
    }

    assert_eq!(test_map.map.entries[0].value, "world");
    assert_eq!(test_map.map.extra_values.len(), 1);
    assert_eq!(test_map.map.extra_values[0].value, "earth");
}

#[test]
#[should_panic]
fn test_append_to_nonexistent_entry() {
    #[derive(Debug)]
    struct TestHeaderMap {
        map: HeaderMap,
    }

    impl TestHeaderMap {
        fn new() -> Self {
            let map = HeaderMap {
                mask: 1,
                indices: Box::new([]),
                entries: vec![], // No entries
                extra_values: vec![],
                danger: Danger::default(),
            };
            TestHeaderMap { map }
        }

        fn entry_mut(&mut self, key: &str) -> Option<&mut OccupiedEntry<'_, String>> {
            None // No entry exists
        }
    }

    let mut test_map = TestHeaderMap::new();
    if let Some(mut occupied) = test_map.entry_mut("not_exists") {
        occupied.append("value".to_string()); // Should panic
    }
}

