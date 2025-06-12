// Answer 0

#[test]
fn test_from_key_occupied_entry() {
    use hashbrown::hash_map::HashMap;
    use std::hash::{BuildHasher, Hasher, Hash};

    // Custom hasher for testing
    struct TestHasher;
    impl BuildHasher for TestHasher {
        type Hasher = TestHasherImpl;

        fn build_hasher(&self) -> Self::Hasher {
            TestHasherImpl
        }
    }

    struct TestHasherImpl;
    impl Hasher for TestHasherImpl {
        fn write(&mut self, _bytes: &[u8]) {
            // No-op
        }
        fn finish(&self) -> u64 {
            42 // Fixed hash value for testing
        }
    }

    let mut map: HashMap<&str, u32, TestHasher> = HashMap::new();
    map.insert("a", 100);
    
    let key = "a";
    let entry = map.raw_entry_mut().from_key(&key);
    
    if let hashbrown::hash_map::RawEntryMut::Occupied(occupied) = entry {
        let mut value_ref = occupied.get_mut();
        *value_ref += 100; // Modify existing value
    } else {
        panic!("Expected occupied entry");
    }

    assert_eq!(map[&"a"], 200);
}

#[test]
fn test_from_key_vacant_entry() {
    use hashbrown::hash_map::HashMap;

    let mut map: HashMap<&str, u32> = HashMap::new();
    let key = "b";
    
    let entry = map.raw_entry_mut().from_key(&key);
    
    if let hashbrown::hash_map::RawEntryMut::Vacant(vacant) = entry {
        vacant.insert(key, 200); // Insert a new key-value pair
    } else {
        panic!("Expected vacant entry");
    }

    assert_eq!(map[&"b"], 200);
}

#[test]
fn test_from_key_non_existing_key() {
    use hashbrown::hash_map::HashMap;

    let mut map: HashMap<&str, i32> = HashMap::new();
    let key = "non_existing_key";
    
    let entry = map.raw_entry_mut().from_key(&key);
    
    if let hashbrown::hash_map::RawEntryMut::Vacant(vacant) = entry {
        vacant.insert(key, 42); // Insert a new key-value pair
    } else {
        panic!("Expected vacant entry");
    }

    assert_eq!(map.get(&key), Some(&42));
}

