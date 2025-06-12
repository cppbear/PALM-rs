// Answer 0

#[test]
fn test_key_mut_occ() {
    struct TestMap<K, V> {
        entries: Vec<(K, V)>,
    }

    let mut occupied_entry = {
        let mut entries = TestMap { entries: vec![(1, "value1")] };
        let index = 0; // Assume we have an occupied entry at index 0
        Entry::Occupied(OccupiedEntry::new(&mut entries.entries, index))
    };

    if let Entry::Occupied(ref mut occupied) = occupied_entry {
        let key_mut = occupied.key_mut();
        *key_mut = 2; // Mutate the key to a new value
    }

    if let Entry::Occupied(ref occupied) = occupied_entry {
        assert_eq!(occupied.key(), &2); // Validate that the key was mutated
    }
}

#[test]
fn test_key_mut_vac() {
    struct TestMap<K, V> {
        entries: Vec<(K, V)>,
    }

    let vacant_entry = {
        let mut entries = TestMap { entries: vec![] };
        Entry::Vacant(VacantEntry {
            map: &mut entries,
            hash: 0, // Dummy hash value
            key: 3, // Initial key value
        })
    };

    let key_mut = vacant_entry.key_mut(); // Get mutable reference to the key
    *key_mut = 4; // Mutate the key

    if let Entry::Vacant(ref vacant) = vacant_entry {
        assert_eq!(vacant.key(), &4); // Validate that the key was mutated
    }
}

