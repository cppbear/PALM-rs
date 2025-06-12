// Answer 0

#[test]
fn test_or_insert_with_vacant_entry() {
    struct TestMap {
        entries: Vec<(String, i32)>,
    }

    impl TestMap {
        fn new() -> Self {
            TestMap { entries: Vec::new() }
        }

        fn insert(&mut self, key: String, value: i32) {
            self.entries.push((key, value));
        }

        fn get_mut(&mut self, key: &String) -> Option<&mut i32> {
            self.entries.iter_mut().find_map(|(k, v)| if k == key { Some(v) } else { None })
        }
    }

    let mut map = TestMap::new();
    let key = String::from("key");
    let empty_entry = Entry::Vacant(VacantEntry {
        map: RefMut::new(&mut map),
        hash: HashValue::default(),
        key: key.clone(),
    });

    let value = empty_entry.or_insert_with(|| 42);
    assert_eq!(*value, 42);
    assert_eq!(map.entries.len(), 1);
    assert_eq!(map.entries[0], (key, 42));
}

#[test]
fn test_or_insert_with_occupied_entry() {
    struct TestMap {
        entries: Vec<(String, i32)>,
    }

    impl TestMap {
        fn new() -> Self {
            TestMap { entries: Vec::new() }
        }

        fn insert(&mut self, key: String, value: i32) {
            self.entries.push((key, value));
        }

        fn get_mut(&mut self, key: &String) -> Option<&mut i32> {
            self.entries.iter_mut().find_map(|(k, v)| if k == key { Some(v) } else { None })
        }
    }

    let mut map = TestMap::new();
    map.insert(String::from("key"), 42);
    let key = String::from("key");

    let occupied_entry = Entry::Occupied(OccupiedEntry {
        entries: &mut map.entries,
        index: hashbrown::hash_table::OccupiedEntry::from_index(0),
    });

    let value = occupied_entry.or_insert_with(|| 100);
    assert_eq!(*value, 42);
    assert_eq!(map.entries.len(), 1);
    assert_eq!(map.entries[0], (key, 42));
}

