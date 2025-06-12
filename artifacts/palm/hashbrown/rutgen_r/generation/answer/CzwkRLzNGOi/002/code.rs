// Answer 0

#[test]
fn test_key_occupied_entry() {
    use hashbrown::HashMap;
    use hashbrown::hash_map::Entry;

    struct OccupiedEntry<'a, K: 'a> {
        key: &'a K,
    }

    impl<'a, K> OccupiedEntry<'a, K> {
        fn key(&self) -> &'a K {
            self.key
        }
    }

    struct VacantEntry<'a, K: 'a> {
        key: &'a K,
    }

    impl<'a, K> VacantEntry<'a, K> {
        fn key(&self) -> &'a K {
            self.key
        }
    }

    enum TestEntry<'a, K: 'a> {
        Occupied(OccupiedEntry<'a, K>),
        Vacant(VacantEntry<'a, K>),
    }

    impl<'a, K> TestEntry<'a, K> {
        pub fn key(&self) -> &'a K {
            match *self {
                TestEntry::Occupied(ref entry) => entry.key(),
                TestEntry::Vacant(ref entry) => entry.key(),
            }
        }
    }

    // Test with an occupied entry
    let key_occupied = "existing_key";
    let occupied_entry = TestEntry::Occupied(OccupiedEntry { key: &key_occupied });
    assert_eq!(occupied_entry.key(), &"existing_key");

    // Test with a vacant entry
    let key_vacant = "nonexistent_key";
    let vacant_entry = TestEntry::Vacant(VacantEntry { key: &key_vacant });
    assert_eq!(vacant_entry.key(), &"nonexistent_key");
}

