// Answer 0

#[test]
fn test_find_existing_entry() {
    struct TestAllocator;
    struct DefaultHashBuilder;
    
    impl DefaultHashBuilder {
        pub fn hash_one(&self, val: &i32) -> u64 {
            *val as u64
        }
    }

    let mut table = HashTable::new_in(TestAllocator);
    let hasher = DefaultHashBuilder;

    // Insert unique values
    table.insert_unique(hasher.hash_one(&1), 1, |val| hasher.hash_one(val));
    table.insert_unique(hasher.hash_one(&2), 2, |val| hasher.hash_one(val));
    table.insert_unique(hasher.hash_one(&3), 3, |val| hasher.hash_one(val));

    // Test finding an existing entry
    assert_eq!(table.find(hasher.hash_one(&2), |&val| val == 2), Some(&2));
}

#[test]
fn test_find_non_existing_entry() {
    struct TestAllocator;
    struct DefaultHashBuilder;

    impl DefaultHashBuilder {
        pub fn hash_one(&self, val: &i32) -> u64 {
            *val as u64
        }
    }

    let mut table = HashTable::new_in(TestAllocator);
    let hasher = DefaultHashBuilder;

    // Insert unique values
    table.insert_unique(hasher.hash_one(&1), 1, |val| hasher.hash_one(val));
    table.insert_unique(hasher.hash_one(&2), 2, |val| hasher.hash_one(val));
    
    // Test finding a non-existing entry
    assert_eq!(table.find(hasher.hash_one(&4), |&val| val == 4), None);
}

