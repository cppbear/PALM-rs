// Answer 0

#[test]
fn test_or_insert_with_empty_entry() {
    struct MockHeaderMap {
        entries: HashMap<HeaderName, HeaderValue>,
    }

    impl MockHeaderMap {
        fn new() -> Self {
            Self {
                entries: HashMap::with_capacity(128),
            }
        }

        fn entry(&mut self, key: &str) -> Entry<&mut Self, HeaderValue> {
            let header_name: HeaderName = key.parse().unwrap();
            if self.entries.contains_key(&header_name) {
                Entry::Occupied(OccupiedEntry {
                    map: self,
                    probe: 0,
                    index: 0,
                })
            } else {
                Entry::Vacant(VacantEntry {
                    map: self,
                    key: header_name,
                    hash: 0, // Assuming a placeholder hash value
                    probe: 0,
                    danger: false,
                })
            }
        }

        fn insert(&mut self, key: HeaderName, value: HeaderValue) {
            self.entries.insert(key, value);
        }
    }

    let mut map = MockHeaderMap::new();
    let res = map.entry("x-hello")
        .or_insert_with(|| "world".parse().unwrap());

    assert_eq!(res, &"world".parse::<HeaderValue>().unwrap());
}

#[test]
fn test_or_insert_with_existing_entry() {
    struct MockHeaderMap {
        entries: HashMap<HeaderName, HeaderValue>,
    }

    impl MockHeaderMap {
        fn new() -> Self {
            Self {
                entries: HashMap::with_capacity(128),
            }
        }

        fn try_insert(&mut self, key: HeaderName, value: HeaderValue) -> Result<(), ()> {
            self.entries.insert(key, value);
            Ok(())
        }

        fn try_entry(&mut self, key: &str) -> Option<Entry<&mut Self, HeaderValue>> {
            let header_name: HeaderName = key.parse().unwrap();
            if self.entries.contains_key(&header_name) {
                Some(Entry::Occupied(OccupiedEntry {
                    map: self,
                    probe: 0,
                    index: 0,
                }))
            } else {
                None
            }
        }
    }

    let mut map = MockHeaderMap::new();
    map.try_insert("host".parse().unwrap(), "world".parse().unwrap()).unwrap();

    let res = map.try_entry("host")
        .unwrap()
        .or_try_insert_with(|| unreachable!())
        .unwrap();

    assert_eq!(res, &"world".parse::<HeaderValue>().unwrap());
}

