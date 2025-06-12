// Answer 0

#[test]
fn test_or_insert_with_key_occupied() {
    use hashbrown::HashMap;
    use std::hash::BuildHasher;

    struct TestHasher;

    impl BuildHasher for TestHasher {
        type Hasher = std::collections::hash_map::DefaultHasher;

        fn build_hasher(&self) -> Self::Hasher {
            std::collections::hash_map::DefaultHasher::new()
        }
    }

    let mut map: HashMap<&str, usize> = HashMap::with_hasher(TestHasher);

    // Insert an initial value to make the entry occupied
    map.insert("key1", 42);

    // Closure to be used for the test
    let closure = |key: &str| key.len();

    // Check that we can retrieve the mutable reference correctly
    let value_mut: &mut usize = map.entry("key1").or_insert_with_key(closure);
    *value_mut += 1; // Modify the value
    assert_eq!(map["key1"], 43);
}

#[test]
fn test_or_insert_with_key_vacant() {
    use hashbrown::HashMap;
    use std::hash::BuildHasher;

    struct TestHasher;

    impl BuildHasher for TestHasher {
        type Hasher = std::collections::hash_map::DefaultHasher;

        fn build_hasher(&self) -> Self::Hasher {
            std::collections::hash_map::DefaultHasher::new()
        }
    }

    let mut map: HashMap<&str, usize> = HashMap::with_hasher(TestHasher);

    // Check that a new value will be added when the key is vacant
    let closure = |key: &str| key.chars().count();

    // Inserting a new occupied value with the length of the key
    let value_mut: &mut usize = map.entry("new_key").or_insert_with_key(closure);
    assert_eq!(*value_mut, 8); // "new_key" has 8 characters
}

#[test]
#[should_panic]
fn test_or_insert_with_key_invalid_key() {
    use hashbrown::HashMap;
    use std::hash::BuildHasher;

    struct TestHasher;

    impl BuildHasher for TestHasher {
        type Hasher = std::collections::hash_map::DefaultHasher;

        fn build_hasher(&self) -> Self::Hasher {
            std::collections::hash_map::DefaultHasher::new()
        }
    }

    let mut map: HashMap<&str, usize> = HashMap::with_hasher(TestHasher);

    // Test panic with a vacant entry and closure that relies on invalid key operations
    let _ = map.entry("panic_key").or_insert_with_key(|key| {
        panic!("This should panic!");
    });
}

