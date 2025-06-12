// Answer 0

#[test]
fn test_shift_remove_full_multiple_elements() {
    struct SimpleHasher;
    
    impl BuildHasher for SimpleHasher {
        type Hasher = std::collections::hash_map::DefaultHasher;

        fn build_hasher(&self) -> Self::Hasher {
            std::collections::hash_map::DefaultHasher::new()
        }
    }

    let mut map: IndexMap<i32, String, SimpleHasher> = IndexMap::with_capacity(5);
    map.insert(1, "one".to_string());
    map.insert(2, "two".to_string());
    map.insert(3, "three".to_string());
    
    // Expecting to remove the key 2 and return (index, key, value)
    assert_eq!(map.shift_remove_full(&2), Some((1, 2, "two".to_string())));
    
    // Verifying that the remaining elements are in correct order
    assert_eq!(map.get(&1), Some(&"one".to_string()));
    assert_eq!(map.get(&3), Some(&"three".to_string()));
}

#[test]
fn test_shift_remove_full_first_element() {
    struct SimpleHasher;
    
    impl BuildHasher for SimpleHasher {
        type Hasher = std::collections::hash_map::DefaultHasher;

        fn build_hasher(&self) -> Self::Hasher {
            std::collections::hash_map::DefaultHasher::new()
        }
    }

    let mut map: IndexMap<i32, String, SimpleHasher> = IndexMap::with_capacity(5);
    map.insert(1, "one".to_string());
    map.insert(2, "two".to_string());
    map.insert(3, "three".to_string());

    // Expecting to remove the first element (key = 1)
    assert_eq!(map.shift_remove_full(&1), Some((0, 1, "one".to_string())));

    // Verifying that the remaining elements are still correct
    assert_eq!(map.get(&2), Some(&"two".to_string()));
    assert_eq!(map.get(&3), Some(&"three".to_string()));
}

#[test]
fn test_shift_remove_full_last_element() {
    struct SimpleHasher;
    
    impl BuildHasher for SimpleHasher {
        type Hasher = std::collections::hash_map::DefaultHasher;

        fn build_hasher(&self) -> Self::Hasher {
            std::collections::hash_map::DefaultHasher::new()
        }
    }

    let mut map: IndexMap<i32, String, SimpleHasher> = IndexMap::with_capacity(5);
    map.insert(1, "one".to_string());
    map.insert(2, "two".to_string());
    map.insert(3, "three".to_string());

    // Expecting to remove the last element (key = 3)
    assert_eq!(map.shift_remove_full(&3), Some((2, 3, "three".to_string())));

    // Verifying that the remaining elements are still correct
    assert_eq!(map.get(&1), Some(&"one".to_string()));
    assert_eq!(map.get(&2), Some(&"two".to_string()));
}

#[test]
fn test_shift_remove_full_no_key() {
    struct SimpleHasher;
    
    impl BuildHasher for SimpleHasher {
        type Hasher = std::collections::hash_map::DefaultHasher;

        fn build_hasher(&self) -> Self::Hasher {
            std::collections::hash_map::DefaultHasher::new()
        }
    }

    let mut map: IndexMap<i32, String, SimpleHasher> = IndexMap::with_capacity(5);
    map.insert(1, "one".to_string());
    map.insert(2, "two".to_string());
    map.insert(3, "three".to_string());

    // Expecting to return None as the key 4 does not exist
    assert_eq!(map.shift_remove_full(&4), None);
}

#[test]
fn test_shift_remove_full_empty_map() {
    struct SimpleHasher;

    impl BuildHasher for SimpleHasher {
        type Hasher = std::collections::hash_map::DefaultHasher;

        fn build_hasher(&self) -> Self::Hasher {
            std::collections::hash_map::DefaultHasher::new()
        }
    }

    let mut map: IndexMap<i32, String, SimpleHasher> = IndexMap::with_capacity(5);
    
    // Expecting to return None as the map is empty
    assert_eq!(map.shift_remove_full(&1), None);
}

