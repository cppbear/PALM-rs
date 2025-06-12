// Answer 0

#[test]
fn test_or_insert_with_occupied_entry() {
    struct TestHeaderMap {
        entries: HashMap<HeaderName, usize>,
    }
    
    impl TestHeaderMap {
        fn entry(&mut self, key: &str) -> Entry<usize> {
            let header_name = HeaderName::try_from(key).unwrap();
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
                    hash: 0,
                    probe: 0,
                    danger: false,
                })
            }
        }
        
        fn insert(&mut self, key: HeaderName, value: usize) {
            self.entries.insert(key, value);
        }
        
        fn get(&self, key: &HeaderName) -> Option<&usize> {
            self.entries.get(key)
        }
    }

    let mut map = TestHeaderMap {
        entries: HashMap::new(),
    };

    map.insert(HeaderName::try_from("content-length").unwrap(), 1);

    let counter = map.entry("content-length").or_insert(0);
    *counter += 1;

    assert_eq!(map.get(&HeaderName::try_from("content-length").unwrap()), Some(&2));
}

#[test]
#[should_panic]
fn test_or_insert_exceeds_max_size() {
    struct TestHeaderMap {
        entries: HashMap<HeaderName, usize>,
    }
    
    impl TestHeaderMap {
        fn entry(&mut self, key: &str) -> Entry<usize> {
            let header_name = HeaderName::try_from(key).unwrap();
            Entry::Vacant(VacantEntry {
                map: self,
                key: header_name,
                hash: 0,
                probe: 0,
                danger: false,
            })
        }

        fn insert(&mut self, key: HeaderName, value: usize) {
            self.entries.insert(key, value);
        }

        fn size(&self) -> usize {
            self.entries.len()
        }
    }

    let mut map = TestHeaderMap {
        entries: HashMap::new(),
    };

    for i in 0..MAX_SIZE as usize + 1 {
        map.entry(&format!("header-{}", i)).or_insert(i);
    }
}

