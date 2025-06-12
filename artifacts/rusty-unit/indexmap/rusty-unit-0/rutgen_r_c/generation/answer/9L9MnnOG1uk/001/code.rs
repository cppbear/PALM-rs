// Answer 0

#[test]
fn test_values_mut_non_empty() {
    struct TestHashBuilder;

    // Implementing necessary trait bounds for HashBuilder.
    impl BuildHasher for TestHashBuilder {
        type Hasher = std::collections::hash_map::RandomState; // Using default state for simplicity
        fn build_hasher(&self) -> Self::Hasher { std::collections::hash_map::RandomState::new() }
    }

    let mut map: IndexMap<i32, String, TestHashBuilder> = IndexMap::with_capacity_and_hasher(10, TestHashBuilder);
    map.insert(1, "one".to_string());
    map.insert(2, "two".to_string());
    
    let mut values_mut_iter = map.values_mut();
    assert_eq!(values_mut_iter.iter.next(), Some(&mut "one".to_string()));
    assert_eq!(values_mut_iter.iter.next(), Some(&mut "two".to_string()));
}

#[test]
fn test_values_mut_empty() {
    struct TestHashBuilder;

    impl BuildHasher for TestHashBuilder {
        type Hasher = std::collections::hash_map::RandomState;
        fn build_hasher(&self) -> Self::Hasher { std::collections::hash_map::RandomState::new() }
    }

    let mut map: IndexMap<i32, String, TestHashBuilder> = IndexMap::with_capacity_and_hasher(0, TestHashBuilder);
    
    let mut values_mut_iter = map.values_mut();
    assert_eq!(values_mut_iter.iter.next(), None);
}

#[should_panic(expected = "some panic case message")]
#[test]
fn test_values_mut_panic() {
    struct TestHashBuilder;

    impl BuildHasher for TestHashBuilder {
        type Hasher = std::collections::hash_map::RandomState;
        fn build_hasher(&self) -> Self::Hasher { std::collections::hash_map::RandomState::new() }
    }

    let mut map: IndexMap<i32, String, TestHashBuilder> = IndexMap::with_capacity_and_hasher(1, TestHashBuilder);
    map.insert(1, "one".to_string());
    
    // This will cause panic if we attempt to mutate an out-of-bounds value.
    let mut values_mut_iter = map.values_mut();
    let val = values_mut_iter.iter.next().unwrap();
    drop(val); // Dropping the mutable reference
    let val2 = values_mut_iter.iter.next().unwrap(); // This should panic as we can no longer borrow.
}

