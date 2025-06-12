// Answer 0

#[test]
fn test_or_insert_with_occupied_entry() {
    use hashbrown::{HashTable, DefaultHashBuilder};
    use std::hash::BuildHasher;

    struct OccupiedEntry<T> {
        value: T,
    }
    
    impl<T> OccupiedEntry<T> {
        fn new(value: T) -> Self {
            OccupiedEntry { value }
        }
    }

    struct MockEntry<'a, T> {
        entry: Option<OccupiedEntry<T>>,
        default: Option<&'a T>,
    }

    impl<'a, T: std::cmp::PartialEq> MockEntry<'a, T> {
        fn or_insert(self, default: T) -> &OccupiedEntry<T> {
            match self.entry {
                Some(entry) => &entry,
                None => {
                    let new_entry = OccupiedEntry::new(default);
                    self.entry.get_or_insert(new_entry);
                }
            }
        }
    }

    let mut table: HashTable<&str> = HashTable::new();
    let hasher = DefaultHashBuilder::default();
    let hasher = |val: &_| hasher.hash_one(val);

    // Setup an occupied entry
    table.insert(hasher(&"poneyland"), "existing_value");
    let existing_entry = MockEntry {
        entry: Some(OccupiedEntry::new("existing_value")),
        default: None,
    };

    // Test that it returns the occupied entry
    let entry = existing_entry.or_insert("new_value"); // this should not panic
    assert_eq!(entry.value, "existing_value");

    // Ensure that the table still contains the original value
    assert!(table.find(hasher(&"poneyland"), |&x| x == "existing_value").is_some());
}

