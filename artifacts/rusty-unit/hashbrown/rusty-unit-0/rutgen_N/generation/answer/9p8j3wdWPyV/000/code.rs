// Answer 0

#[test]
fn test_entry_occupied_get() {
    use hashbrown::hash_table::Entry;
    use hashbrown::{HashTable, DefaultHashBuilder};
    use std::hash::BuildHasher;
    
    struct Hasher {
        hasher: DefaultHashBuilder,
    }

    impl Hasher {
        fn new() -> Self {
            Self {
                hasher: DefaultHashBuilder::default(),
            }
        }

        fn hash_one(&self, val: &str) -> u64 {
            self.hasher.hash_one(val)
        }
    }

    let mut table: HashTable<&str> = HashTable::new();
    let hasher = Hasher::new();
    table.insert_unique(hasher.hash_one("poneyland"), "poneyland", |val| hasher.hash_one(val));
    
    match table.entry(hasher.hash_one("poneyland"), |&x| x == "poneyland", |val| hasher.hash_one(val)) {
        Entry::Vacant(_) => panic!(),
        Entry::Occupied(entry) => assert_eq!(entry.get(), &"poneyland"),
    }
}

#[test]
#[should_panic]
fn test_entry_vacant_get() {
    use hashbrown::hash_table::Entry;
    use hashbrown::{HashTable, DefaultHashBuilder};
    use std::hash::BuildHasher;

    struct Hasher {
        hasher: DefaultHashBuilder,
    }

    impl Hasher {
        fn new() -> Self {
            Self {
                hasher: DefaultHashBuilder::default(),
            }
        }

        fn hash_one(&self, val: &str) -> u64 {
            self.hasher.hash_one(val)
        }
    }

    let mut table: HashTable<&str> = HashTable::new();
    let hasher = Hasher::new();

    match table.entry(hasher.hash_one("poneyland"), |&x| x == "poneyland", |val| hasher.hash_one(val)) {
        Entry::Vacant(_) => panic!(),
        Entry::Occupied(_) => {}
    }
}

