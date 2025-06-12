// Answer 0

#[test]
fn test_or_insert_vacant_entry() {
    use hashbrown::HashMap;
    use std::collections::hash_map::DefaultHasher;
    use std::hash::BuildHasher;

    struct MyHasher {
        hasher: DefaultHasher,
    }

    impl BuildHasher for MyHasher {
        type Hash = std::hash::Hash;

        type Hasher = DefaultHasher;

        fn build_hasher(&self) -> Self::Hasher {
            DefaultHasher::new()
        }
    }

    let mut map: HashMap<&str, u32> = HashMap::new();

    let raw_entry_mut = map.raw_entry_mut().from_key("poneyland");
    let (key, value) = raw_entry_mut.or_insert("poneyland", 3);
    
    assert_eq!(*key, "poneyland");
    assert_eq!(*value, 3);
}

#[test]
fn test_or_insert_multiple_vacant_entries() {
    use hashbrown::HashMap;
    use std::collections::hash_map::DefaultHasher;
    use std::hash::BuildHasher;

    struct MyHasher {
        hasher: DefaultHasher,
    }

    impl BuildHasher for MyHasher {
        type Hash = std::hash::Hash;

        type Hasher = DefaultHasher;

        fn build_hasher(&self) -> Self::Hasher {
            DefaultHasher::new()
        }
    }

    let mut map: HashMap<&str, u32> = HashMap::new();

    let raw_entry_mut_first = map.raw_entry_mut().from_key("poneyland");
    let (key_first, value_first) = raw_entry_mut_first.or_insert("poneyland", 3);
    
    assert_eq!(*key_first, "poneyland");
    assert_eq!(*value_first, 3);

    let raw_entry_mut_second = map.raw_entry_mut().from_key("unicornland");
    let (key_second, value_second) = raw_entry_mut_second.or_insert("unicornland", 5);
    
    assert_eq!(*key_second, "unicornland");
    assert_eq!(*value_second, 5);
}

#[test]
fn test_or_insert_existing_key_updates_value() {
    use hashbrown::HashMap;

    let mut map: HashMap<&str, u32> = HashMap::new();
    
    map.raw_entry_mut().from_key("poneyland").or_insert("poneyland", 3);
    let (key, value) = map.raw_entry_mut().from_key("poneyland").or_insert("poneyland", 10);
    
    assert_eq!(*key, "poneyland");
    assert_eq!(*value, 3); // Should still be 3, not updated because the entry is already occupied
    *value *= 2;
    assert_eq!(map["poneyland"], 6);
}

