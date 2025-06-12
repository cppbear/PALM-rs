// Answer 0

#[test]
fn test_get_mut_should_return_mutable_reference() {
    struct TestHeaderMap {
        map: HeaderMap<String>,
    }

    impl TestHeaderMap {
        fn new() -> Self {
            TestHeaderMap {
                map: HeaderMap {
                    mask: 0,
                    indices: Box::from([]),
                    entries: vec![],
                    extra_values: vec![],
                    danger: Danger::default(),
                },
            }
        }

        fn insert(&mut self, key: HeaderName, value: String) {
            self.map.entries.push(Bucket {
                hash: 0,
                key,
                value,
                links: None,
            });
        }

        fn entry(&mut self, key: &str) -> Option<OccupiedEntry<String>> {
            // Simple simulation of finding an entry
            self.map.entries.iter_mut().position(|bucket| bucket.key == key).map(|index| {
                OccupiedEntry {
                    map: &mut self.map,
                    probe: 0,
                    index,
                }
            })
        }
    }

    let mut test_map = TestHeaderMap::new();
    test_map.insert(HeaderName::from("host"), "hello.world".to_string());

    if let Some(mut entry) = test_map.entry("host") {
        entry.get_mut().push_str("-2");
        assert_eq!(entry.get(), &"hello.world-2");
    }
}

#[test]
#[should_panic]
fn test_get_mut_should_panic_if_no_values() {
    struct TestHeaderMap {
        map: HeaderMap<String>,
    }

    impl TestHeaderMap {
        fn new() -> Self {
            TestHeaderMap {
                map: HeaderMap {
                    mask: 0,
                    indices: Box::from([]),
                    entries: vec![],
                    extra_values: vec![],
                    danger: Danger::default(),
                },
            }
        }

        fn entry(&mut self, _key: &str) -> Option<OccupiedEntry<String>> {
            None
        }
    }

    let mut test_map = TestHeaderMap::new();

    if let Some(mut entry) = test_map.entry("host") {
        entry.get_mut(); // This should panic
    }
}

