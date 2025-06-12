// Answer 0

#[test]
fn test_insert_entry_occupied() {
    struct MyKey;
    struct MyValue;

    enum Entry<'a, K, V> {
        Occupied(OccupiedEntry<'a, K, V>),
        Vacant(VacantEntry<'a, K, V>),
    }

    struct OccupiedEntry<'a, K, V> {
        value: Option<V>,
    }

    struct VacantEntry<'a, K, V> {
        value: Option<V>,
    }

    impl<'a, K, V> OccupiedEntry<'a, K, V> {
        fn insert(&mut self, value: V) {
            self.value = Some(value);
        }
    }

    impl<'a, K, V> VacantEntry<'a, K, V> {
        fn insert_entry(self, value: V) -> OccupiedEntry<'a, K, V> {
            OccupiedEntry { value: Some(value) }
        }
    }
    
    let mut entry = Entry::Occupied(OccupiedEntry { value: None });
    if let Entry::Occupied(ref mut occupied_entry) = entry {
        occupied_entry.insert(MyValue);
    }

    if let Entry::Occupied(occupied_entry) = entry {
        assert!(occupied_entry.value.is_some());
    }
}

#[test]
fn test_insert_entry_vacant() {
    struct MyKey;
    struct MyValue;

    enum Entry<'a, K, V> {
        Occupied(OccupiedEntry<'a, K, V>),
        Vacant(VacantEntry<'a, K, V>),
    }

    struct OccupiedEntry<'a, K, V> {
        value: Option<V>,
    }

    struct VacantEntry<'a, K, V> {
        value: Option<V>,
    }

    impl<'a, K, V> OccupiedEntry<'a, K, V> {
        fn insert(&mut self, value: V) {
            self.value = Some(value);
        }
    }

    impl<'a, K, V> VacantEntry<'a, K, V> {
        fn insert_entry(self, value: V) -> OccupiedEntry<'a, K, V> {
            OccupiedEntry { value: Some(value) }
        }
    }

    let entry: Entry<MyKey, MyValue> = Entry::Vacant(VacantEntry { value: None });
    let occupied_entry = match entry {
        Entry::Vacant(vacant_entry) => vacant_entry.insert_entry(MyValue),
        _ => panic!("Expected VacantEntry"),
    };

    assert!(occupied_entry.value.is_some());
}

