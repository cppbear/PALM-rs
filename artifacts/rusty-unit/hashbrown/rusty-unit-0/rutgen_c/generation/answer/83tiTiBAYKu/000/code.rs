// Answer 0

#[test]
fn test_entry_ref_occupied() {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};

    struct TestHashBuilder;

    impl BuildHasher for TestHashBuilder {
        type Hasher = DefaultHasher;

        fn build_hasher(&self) -> Self::Hasher {
            DefaultHasher::new()
        }
    }

    let mut map: HashMap<String, usize, TestHashBuilder> = HashMap::new();
    map.insert("apple".to_string(), 1);
    map.insert("banana".to_string(), 2);

    let key = "apple";
    match map.entry_ref(key) {
        EntryRef::Occupied(_) => (),
        EntryRef::Vacant(_) => panic!("Expected occupied entry"),
    }
}

#[test]
fn test_entry_ref_vacant() {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};

    struct TestHashBuilder;

    impl BuildHasher for TestHashBuilder {
        type Hasher = DefaultHasher;

        fn build_hasher(&self) -> Self::Hasher {
            DefaultHasher::new()
        }
    }

    let mut map: HashMap<String, usize, TestHashBuilder> = HashMap::new();
    
    let key = "orange";
    match map.entry_ref(key) {
        EntryRef::Vacant(_) => (),
        EntryRef::Occupied(_) => panic!("Expected vacant entry"),
    }
}

#[test]
fn test_entry_ref_after_insertion() {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};

    struct TestHashBuilder;

    impl BuildHasher for TestHashBuilder {
        type Hasher = DefaultHasher;

        fn build_hasher(&self) -> Self::Hasher {
            DefaultHasher::new()
        }
    }

    let mut map: HashMap<String, usize, TestHashBuilder> = HashMap::new();
    
    let key = "grape";
    // Insert initial value
    map.insert(key.to_string(), 0);
    
    // Modify entry via entry_ref
    {
        let counter = map.entry_ref(key).unwrap();
        if let EntryRef::Occupied(entry) = counter {
            *entry.value() += 1;
        }
    }

    assert_eq!(map.get(key), Some(&1));
}

