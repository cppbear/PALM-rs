// Answer 0

#[cfg(test)]
fn test_or_insert_with() {
    use hashbrown::{HashTable, DefaultHashBuilder};
    use std::hash::BuildHasher;

    struct EntryWrapper<'a, T, A> {
        entry: hashbrown::entry::Entry<'a, T, A>,
    }

    impl<'a, T, A> EntryWrapper<'a, T, A> {
        fn or_insert_with(self, default: impl FnOnce() -> T) -> OccupiedEntry<'a, T, A> {
            match self.entry {
                Entry::Occupied(entry) => entry,
                Entry::Vacant(entry) => entry.insert(default()),
            }
        }
    }

    let mut table: HashTable<String> = HashTable::new();
    let hasher = DefaultHashBuilder::default();
    let hasher = |val: &_| hasher.hash_one(val);

    // Test inserting a new value
    let entry = EntryWrapper { entry: table.entry(hasher("poneyland")) };
    entry.or_insert_with(|| "poneyland".to_string());

    assert!(table.find(hasher(&"poneyland"), |x| x == "poneyland").is_some());

    // Test inserting a value when the entry is already occupied
    let entry = EntryWrapper { entry: table.entry(hasher("poneyland")) };
    let occupied_entry = entry.or_insert_with(|| "should_not_insert".to_string());

    assert_eq!(occupied_entry.get(), "poneyland");
}

#[test]
fn run_tests() {
    test_or_insert_with();
}

