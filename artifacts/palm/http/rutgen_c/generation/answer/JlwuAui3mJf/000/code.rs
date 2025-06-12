// Answer 0

#[test]
fn test_key_on_occupied_entry() {
    #[derive(Debug)]
    struct TestHeaderMap {
        entries: Vec<(HeaderName, usize)>,
    }

    impl TestHeaderMap {
        fn new() -> Self {
            Self { entries: vec![] }
        }

        fn insert(&mut self, key: HeaderName, value: usize) {
            self.entries.push((key, value));
        }

        fn entry(&mut self, key: &str) -> Entry<'_, usize> {
            let header_name = HeaderName {
                inner: Repr::Custom(key.to_string()),
            };

            if let Some((index, _)) = self.entries.iter().enumerate().find(|(i, (k, _))| k == &header_name) {
                Entry::Occupied(OccupiedEntry {
                    map: self,
                    index,
                    probe: index,
                })
            } else {
                Entry::Vacant(VacantEntry {
                    map: self,
                    key: header_name,
                    hash: 0,
                    probe: 0,
                    danger: false,
                })
            }
        }
    }

    #[derive(Debug, Clone, Eq, PartialEq, Hash)]
    struct Repr<T> {
        value: T,
    }

    let mut map = TestHeaderMap::new();
    let key_name = HeaderName {
        inner: Repr::Custom("x-hello".to_string()),
    };
    map.insert(key_name.clone(), 42);

    let entry = map.entry("x-hello");
    assert_eq!(entry.key(), &key_name);
}

#[test]
fn test_key_on_vacant_entry() {
    #[derive(Debug)]
    struct TestHeaderMap {
        entries: Vec<(HeaderName, usize)>,
    }

    impl TestHeaderMap {
        fn new() -> Self {
            Self { entries: vec![] }
        }

        fn entry(&mut self, key: &str) -> Entry<'_, usize> {
            let header_name = HeaderName {
                inner: Repr::Custom(key.to_string()),
            };

            Entry::Vacant(VacantEntry {
                map: self,
                key: header_name,
                hash: 0,
                probe: 0,
                danger: false,
            })
        }
    }

    #[derive(Debug, Clone, Eq, PartialEq, Hash)]
    struct Repr<T> {
        value: T,
    }

    let mut map = TestHeaderMap::new();
    let key_name = HeaderName {
        inner: Repr::Custom("x-hello".to_string()),
    };

    let entry = map.entry("x-hello");
    assert_eq!(entry.key(), &key_name);
}

