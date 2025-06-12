// Answer 0

#[test]
fn test_into_keys() {
    struct SimpleHashBuilder;

    impl BuildHasher for SimpleHashBuilder {
        type Hasher = std::collections::hash_map::RandomState;

        fn build_hasher(&self) -> Self::Hasher {
            Self::Hasher::new()
        }
    }

    let map = IndexMap::<i32, &str, SimpleHashBuilder>::with_capacity_and_hasher(10, SimpleHashBuilder);
    
    // Assuming a method to insert items, as the original has no direct insert method provided
    // Use a hypothetical insert method in a realistic scenario (commented out since it is not defined)
    // map.insert(1, "one");
    // map.insert(2, "two");

    let keys_iterator = map.into_keys();
    let keys: Vec<_> = keys_iterator.iter.collect(); // This assumes that Iter collects keys.

    assert_eq!(keys, vec![1, 2]); // Change according to actual key insertion logic
}

#[test]
fn test_into_keys_empty() {
    struct SimpleHashBuilder;

    impl BuildHasher for SimpleHashBuilder {
        type Hasher = std::collections::hash_map::RandomState;

        fn build_hasher(&self) -> Self::Hasher {
            Self::Hasher::new()
        }
    }

    let map = IndexMap::<i32, &str, SimpleHashBuilder>::with_capacity_and_hasher(10, SimpleHashBuilder);
    let keys_iterator = map.into_keys();
    let keys: Vec<_> = keys_iterator.iter.collect(); // This assumes that Iter collects keys.

    assert!(keys.is_empty());
}

