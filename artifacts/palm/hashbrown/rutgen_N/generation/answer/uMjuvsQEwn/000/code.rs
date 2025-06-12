// Answer 0

#[test]
fn test_find_existing_entry() {
    use hashbrown::{HashTable, DefaultHashBuilder};
    use std::hash::BuildHasher;

    struct Hasher {
        hasher: DefaultHashBuilder,
    }

    impl Hasher {
        fn hash_one(&self, val: &i32) -> u64 {
            self.hasher.hash_one(val)
        }
    }

    let mut table = HashTable::new();
    let hasher = Hasher {
        hasher: DefaultHashBuilder::default(),
    };
    
    table.insert_unique(hasher.hash_one(&1), 1, |val| hasher.hash_one(val));
    table.insert_unique(hasher.hash_one(&2), 2, |val| hasher.hash_one(val));
    
    assert_eq!(table.find(hasher.hash_one(&2), |&val| val == 2), Some(&2));
}

#[test]
fn test_find_non_existing_entry() {
    use hashbrown::{HashTable, DefaultHashBuilder};
    use std::hash::BuildHasher;

    struct Hasher {
        hasher: DefaultHashBuilder,
    }

    impl Hasher {
        fn hash_one(&self, val: &i32) -> u64 {
            self.hasher.hash_one(val)
        }
    }

    let mut table = HashTable::new();
    let hasher = Hasher {
        hasher: DefaultHashBuilder::default(),
    };
    
    table.insert_unique(hasher.hash_one(&1), 1, |val| hasher.hash_one(val));
    table.insert_unique(hasher.hash_one(&2), 2, |val| hasher.hash_one(val));
    
    assert_eq!(table.find(hasher.hash_one(&4), |&val| val == 4), None);
}

