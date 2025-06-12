// Answer 0

#[test]
fn test_or_insert_with_key_vacant_entry() {
    use std::hash::Hasher;
    use hashbrown::HashMap;

    struct DummyHasher;
    impl BuildHasher for DummyHasher {
        type Hasher = std::collections::hash_map::RandomState;
        
        fn build_hasher(&self) -> Self::Hasher {
            std::collections::hash_map::RandomState::new()
        }
    }

    let mut map: HashMap<&str, usize, DummyHasher> = HashMap::new();
    
    let entry = Entry::Vacant(VacantEntry {
        hash: 0,
        key: "example_key",
        table: &mut map,
    });

    let value = entry.or_insert_with_key(|key| key.chars().count());
    assert_eq!(*value, 12); // The length of "example_key" is 12
}

#[test]
fn test_or_insert_with_key_existing_entry() {
    use std::hash::Hasher;
    use hashbrown::HashMap;

    struct DummyHasher;
    impl BuildHasher for DummyHasher {
        type Hasher = std::collections::hash_map::RandomState;
        
        fn build_hasher(&self) -> Self::Hasher {
            std::collections::hash_map::RandomState::new()
        }
    }

    let mut map: HashMap<&str, usize, DummyHasher> = HashMap::new();
    map.insert("existing_key", 10);

    let entry = Entry::Occupied(OccupiedEntry {
        hash: 0,
        elem: Bucket::new(("existing_key", 10)),
        table: &mut map,
    });

    let value = entry.or_insert_with_key(|key| key.chars().count());
    assert_eq!(*value, 10); // The existing value should not change
}

