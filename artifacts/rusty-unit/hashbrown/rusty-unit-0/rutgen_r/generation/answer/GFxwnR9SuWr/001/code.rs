// Answer 0

#[test]
fn test_insert_unique() {
    use hashbrown::{HashTable, DefaultHashBuilder};
    use std::hash::BuildHasher;

    struct TestHashBuilder;

    impl BuildHasher for TestHashBuilder {
        type Hasher = std::hash::rustc_hash::RustcHasher;

        fn build_hasher(&self) -> Self::Hasher {
            DefaultHashBuilder::default().build_hasher()
        }
    }

    let mut v = HashTable::new();
    let hasher = TestHashBuilder;
    let hash_one = |val: &u64| hasher.build_hasher().hash_one(val);

    let entry_1 = v.insert_unique(hash_one(&1), 1, hash_one);
    assert_eq!(entry_1.hash, hash_one(&1));
    
    let entry_2 = v.insert_unique(hash_one(&2), 2, hash_one);
    assert_eq!(entry_2.hash, hash_one(&2));
    
    let entry_3 = v.insert_unique(hash_one(&3), 3, hash_one);
    assert_eq!(entry_3.hash, hash_one(&3));
}

#[test]
#[should_panic]
fn test_insert_unique_panic_on_key_collision() {
    use hashbrown::{HashTable, DefaultHashBuilder};
    use std::hash::BuildHasher;

    struct TestHashBuilder;

    impl BuildHasher for TestHashBuilder {
        type Hasher = std::hash::rustc_hash::RustcHasher;

        fn build_hasher(&self) -> Self::Hasher {
            DefaultHashBuilder::default().build_hasher()
        }
    }

    let mut v = HashTable::new();
    let hasher = TestHashBuilder;
    let hash_one = |val: &u64| hasher.build_hasher().hash_one(val);

    // Insert the first element
    v.insert_unique(hash_one(&1), 1, hash_one);

    // Attempt to insert with the same hash which may trigger a panic due to re-insert
    v.insert_unique(hash_one(&1), 2, hash_one);
}

