// Answer 0

#[test]
fn test_insert_entry() {
    use hashbrown::HashMap;
    use hashbrown::hash_map::Entry;
    use std::hash::BuildHasherDefault;
    use std::collections::hash_map::RandomState;

    struct CustomHasher {
        state: BuildHasherDefault<RandomState>,
    }

    impl Default for CustomHasher {
        fn default() -> Self {
            Self {
                state: BuildHasherDefault::default(),
            }
        }
    }

    let mut map: HashMap<&str, u32, CustomHasher> = HashMap::with_hasher(CustomHasher::default());

    // Test inserting a new entry into a vacant position
    if let Entry::Vacant(v) = map.entry("poneyland") {
        let o = v.insert_entry(37);
        assert_eq!(o.get(), &37);
    }

    // Test inserting another entry
    if let Entry::Vacant(v) = map.entry("unicornland") {
        let o = v.insert_entry(42);
        assert_eq!(o.get(), &42);
    }

    // Verify the entries exist with expected values
    assert_eq!(map.get("poneyland"), Some(&37));
    assert_eq!(map.get("unicornland"), Some(&42));

    // Edge case: Attempting to insert into the same key should return the occupied entry
    if let Entry::Occupied(mut o) = map.entry("poneyland") {
        let new_value = o.insert_entry(50);
        assert_eq!(new_value.get(), &50);
        assert_eq!(o.get(), &50);
    }
}

