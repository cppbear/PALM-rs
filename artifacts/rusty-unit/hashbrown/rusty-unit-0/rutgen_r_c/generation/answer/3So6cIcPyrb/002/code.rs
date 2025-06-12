// Answer 0

#[test]
fn test_or_insert_with_occupied_entry() {
    use hashbrown::{HashTable, DefaultHashBuilder};
    use std::hash::BuildHasher;

    struct TestHashTable {
        table: HashTable<String>,
        hasher: DefaultHashBuilder,
    }

    impl TestHashTable {
        fn new() -> Self {
            let mut table = HashTable::new();
            let hasher = DefaultHashBuilder::default();
            TestHashTable { table, hasher }
        }

        fn insert_entry(&mut self, key: &str) {
            let hash = self.hasher.hash_one(&key);
            self.table.insert_unique(hash, key.to_string(), |val| val);
        }

        fn get_entry(&mut self, key: &str) -> Entry<String, DefaultHashBuilder> {
            let hash = self.hasher.hash_one(&key);
            self.table.entry(hash, |x| x == key, |val| self.hasher.hash_one(val))
        }
    }

    let mut test_table = TestHashTable::new();
    test_table.insert_entry("poneyland");

    let entry = test_table.get_entry("poneyland");
    match entry {
        Entry::Occupied(mut occupied_entry) => {
            let result = occupied_entry.or_insert_with(|| "default_value".to_string());
            assert_eq!(result, occupied_entry);
        }
        Entry::Vacant(_) => panic!("Expected an occupied entry"),
    }
}

