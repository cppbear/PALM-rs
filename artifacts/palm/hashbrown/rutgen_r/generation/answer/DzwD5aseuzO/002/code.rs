// Answer 0

#[test]
fn test_or_insert_with_occupied_entry() {
    use hashbrown::HashMap;
    use std::hash::BuildHasherDefault;
    
    // Custom struct to simulate usage
    struct MyHasher;
    impl BuildHasher for MyHasher {
        type Hasher = std::collections::hash_map::DefaultHasher;

        fn build_hasher(&self) -> Self::Hasher {
            std::collections::hash_map::DefaultHasher::new()
        }
    }
    
    let mut map: HashMap<&str, String, BuildHasherDefault<MyHasher>> = HashMap::default();
    map.insert("poneyland", "hoho".to_string());

    // We should get a mutable reference to the key and value for an occupied entry
    let (key, value) = map.raw_entry_mut().from_key("poneyland").or_insert_with(|| {
        panic!("This panic should not occur since the entry is occupied.")
    });
    
    assert_eq!(*key, "poneyland");
    assert_eq!(*value, "hoho".to_string());
}

#[test]
#[should_panic]
fn test_or_insert_with_vacant_entry_panics() {
    use hashbrown::HashMap;
    use std::hash::BuildHasherDefault;
    
    // Custom struct to simulate usage
    struct MyHasher;
    impl BuildHasher for MyHasher {
        type Hasher = std::collections::hash_map::DefaultHasher;

        fn build_hasher(&self) -> Self::Hasher {
            std::collections::hash_map::DefaultHasher::new()
        }
    }
    
    let mut map: HashMap<&str, String, BuildHasherDefault<MyHasher>> = HashMap::default();

    // This will panic as there is no entry present
    map.raw_entry_mut().from_key("poneyland").or_insert_with(|| {
        ("poneyland", "hoho".to_string())
    });
}

