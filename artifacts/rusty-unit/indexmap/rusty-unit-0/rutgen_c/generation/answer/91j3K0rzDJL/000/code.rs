// Answer 0

#[test]
fn test_shift_insert_within_bounds() {
    use std::collections::hash_map::RandomState;

    struct Indices {
        // minimal implementation details as an example
        count: usize,
    }

    struct Entries<K, V> {
        entries: Vec<(K, V)>,
    }

    impl<K, V> Entries<K, V> {
        fn new() -> Self {
            Entries { entries: Vec::new() }
        }
    }

    let mut indices = Indices { count: 0 };
    let mut entries = Entries::new();
    let mut map = RefMut {
        indices: &mut indices,
        entries: &mut entries,
    };

    let hash_builder = RandomState::new();
    let key = "test_key";
    let value = "test_value";

    let raw_entry = RawVacantEntryMut {
        map: map,
        hash_builder: &hash_builder,
    };

    let (inserted_key, inserted_value) = raw_entry.shift_insert(0, key, value);
    assert_eq!(*inserted_key, key);
    assert_eq!(*inserted_value, value);
}

#[test]
#[should_panic]
fn test_shift_insert_out_of_bounds() {
    use std::collections::hash_map::RandomState;

    struct Indices {
        count: usize,
    }

    struct Entries<K, V> {
        entries: Vec<(K, V)>,
    }

    impl<K, V> Entries<K, V> {
        fn new() -> Self {
            Entries { entries: Vec::new() }
        }
    }

    let mut indices = Indices { count: 0 };
    let mut entries = Entries::new();
    let mut map = RefMut {
        indices: &mut indices,
        entries: &mut entries,
    };

    let hash_builder = RandomState::new();
    let raw_entry = RawVacantEntryMut {
        map: map,
        hash_builder: &hash_builder,
    };

    // Trying to insert at index 1 when there are no elements should panic
    raw_entry.shift_insert(1, "key", "value");
}

