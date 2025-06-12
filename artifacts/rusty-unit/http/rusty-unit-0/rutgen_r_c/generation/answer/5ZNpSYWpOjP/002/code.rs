// Answer 0

#[test]
fn test_or_try_insert_occupied_entry() {
    #[derive(Debug)]
    struct HeaderMap<T> {
        entries: Vec<(HeaderName, T)>,
        max_size: usize,
    }

    #[derive(Debug)]
    struct HeaderName(String);

    impl<T> HeaderMap<T> {
        fn new(max_size: usize) -> Self {
            HeaderMap {
                entries: Vec::new(),
                max_size,
            }
        }

        fn entry(&mut self, key: HeaderName) -> Entry<T> {
            if let Some(pos) = self.entries.iter().position(|(k, _)| *k == key) {
                Entry::Occupied(OccupiedEntry {
                    map: self,
                    probe: pos,
                    index: pos,
                })
            } else {
                Entry::Vacant(VacantEntry {
                    map: self,
                    key,
                    // Placeholder values for hashing and probing
                    hash: 0,
                    probe: 0,
                    danger: false,
                })
            }
        }

        fn insert(&mut self, key: HeaderName, value: T) {
            self.entries.push((key, value));
        }
    }

    impl<'a, T> OccupiedEntry<'a, T> {
        fn into_mut(self) -> &'a mut T {
            &mut self.map.entries[self.index].1
        }
    }

    let mut map = HeaderMap::new(10);
    let key = HeaderName("test-header".to_string());
    map.insert(key.clone(), 42);

    if let Entry::Occupied(entry) = map.entry(key) {
        let result = entry.or_try_insert(100);
        assert_eq!(*result.unwrap(), 42); // Should return the existing value
    } else {
        panic!("Expected OccupiedEntry, but got VacantEntry");
    }
}

