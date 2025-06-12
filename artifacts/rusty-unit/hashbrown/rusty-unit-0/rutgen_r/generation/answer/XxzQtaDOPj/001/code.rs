// Answer 0

#[test]
fn test_or_insert_vacant_entry() {
    use hashbrown::HashMap;
    use std::hash::BuildHasher;

    struct MyHasher;
    impl BuildHasher for MyHasher {
        type Hasher = std::collections::hash_map::DefaultHasher;
        fn build_hasher(&self) -> Self::Hasher {
            std::collections::hash_map::DefaultHasher::new()
        }
    }

    let mut map: HashMap<&str, u32, MyHasher> = HashMap::new();

    // Test inserting into a vacant entry
    let value = map.entry("empty_key").or_insert(42);
    assert_eq!(*value, 42);
    assert_eq!(map["empty_key"], 42);
}

#[test]
fn test_or_insert_existing_entry() {
    use hashbrown::HashMap;
    use std::hash::BuildHasher;

    struct MyHasher;
    impl BuildHasher for MyHasher {
        type Hasher = std::collections::hash_map::DefaultHasher;
        fn build_hasher(&self) -> Self::Hasher {
            std::collections::hash_map::DefaultHasher::new()
        }
    }

    let mut map: HashMap<&str, u32, MyHasher> = HashMap::new();
    
    // Insert an initial value
    map.insert("existing_key", 5);
    
    // Test modifying an existing entry
    *map.entry("existing_key").or_insert(10) *= 2;
    assert_eq!(map["existing_key"], 10);
}

#[test]
#[should_panic]
fn test_or_insert_panic_on_invalid_key() {
    use hashbrown::HashMap;
    use std::hash::BuildHasher;

    struct MyHasher;
    impl BuildHasher for MyHasher {
        type Hasher = std::collections::hash_map::DefaultHasher;
        fn build_hasher(&self) -> Self::Hasher {
            std::collections::hash_map::DefaultHasher::new()
        }
    }

    let mut map: HashMap<&str, u32, MyHasher> = HashMap::new();
    
    // This will panic because we are trying to access a vacant entry without inserting
    let _value = map.entry("nonexistent_key").or_insert(10);
    assert_eq!(map["nonexistent_key"], 10);
    // Attempting to access a non-inserted key
    let _panic_value = map["another_nonexistent_key"];
}

