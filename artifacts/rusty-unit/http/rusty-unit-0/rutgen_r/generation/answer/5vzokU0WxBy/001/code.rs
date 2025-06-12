// Answer 0

#[test]
fn test_try_insert_entry_success() {
    struct HeaderMap {
        map: Vec<(String, String)>,
    }

    struct VacantEntry<'a> {
        map: &'a mut HeaderMap,
        key: String,
    }

    struct OccupiedEntry<'a, T> {
        map: &'a mut HeaderMap,
        index: usize,
        probe: usize,
    }

    impl HeaderMap {
        fn new() -> Self {
            HeaderMap { map: Vec::new() }
        }

        fn try_entry(&mut self, key: &str) -> Result<VacantEntry, ()> {
            if self.map.iter().any(|(k, _)| k == key) {
                Err(())
            } else {
                Ok(VacantEntry {
                    map: self,
                    key: key.to_string(),
                })
            }
        }

        fn try_insert_phase_two(&mut self, key: String, value: String) -> Result<usize, ()> {
            self.map.push((key.clone(), value));
            Ok(self.map.len() - 1)
        }
    }

    impl VacantEntry<'_> {
        fn try_insert_entry(self, value: String) -> Result<OccupiedEntry<'_, String>, ()> {
            let index = self.map.try_insert_phase_two(self.key, value)?;
            Ok(OccupiedEntry {
                map: self.map,
                index,
                probe: 0, // Placeholder for probe value
            })
        }
    }

    impl OccupiedEntry<'_, String> {
        fn insert(&mut self, value: String) {
            self.map.map[self.index].1 = value;
        }
    }

    let mut map = HeaderMap::new();
    let vacant_entry = map.try_entry("x-hello").unwrap();
    let mut occupied_entry = vacant_entry.try_insert_entry("world".to_string()).unwrap();
    occupied_entry.insert("world2".to_string());

    assert_eq!(map.map[occupied_entry.index].1, "world2");
}

#[test]
#[should_panic]
fn test_try_insert_entry_failure_due_to_duplicate_key() {
    struct HeaderMap {
        map: Vec<(String, String)>,
    }

    struct VacantEntry<'a> {
        map: &'a mut HeaderMap,
        key: String,
    }

    struct OccupiedEntry<'a, T> {
        map: &'a mut HeaderMap,
        index: usize,
        probe: usize,
    }

    impl HeaderMap {
        fn new() -> Self {
            HeaderMap { map: Vec::new() }
        }

        fn try_entry(&mut self, key: &str) -> Result<VacantEntry, ()> {
            if self.map.iter().any(|(k, _)| k == key) {
                Err(())
            } else {
                Ok(VacantEntry {
                    map: self,
                    key: key.to_string(),
                })
            }
        }

        fn try_insert_phase_two(&mut self, key: String, value: String) -> Result<usize, ()> {
            self.map.push((key.clone(), value));
            Ok(self.map.len() - 1)
        }
    }

    impl VacantEntry<'_> {
        fn try_insert_entry(self, value: String) -> Result<OccupiedEntry<'_, String>, ()> {
            let index = self.map.try_insert_phase_two(self.key, value)?;
            Ok(OccupiedEntry {
                map: self.map,
                index,
                probe: 0, // Placeholder for probe value
            })
        }
    }

    impl OccupiedEntry<'_, String> {
        fn insert(&mut self, value: String) {
            self.map.map[self.index].1 = value;
        }
    }

    let mut map = HeaderMap::new();
    let vacant_entry = map.try_entry("x-hello").unwrap();
    let _ = vacant_entry.try_insert_entry("world".to_string()).unwrap();
    
    // This will panic because "x-hello" already exists
    let _ = map.try_entry("x-hello").unwrap().try_insert_entry("new_value".to_string());
}

