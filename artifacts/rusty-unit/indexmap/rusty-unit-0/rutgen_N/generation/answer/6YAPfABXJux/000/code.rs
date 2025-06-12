// Answer 0

#[test]
fn test_hash_with_string_key() {
    use std::hash::{Hash, Hasher};
    use std::collections::hash_map::DefaultHasher;

    struct HashBuilder;

    impl HashBuilder {
        fn build_hasher(&self) -> DefaultHasher {
            DefaultHasher::new()
        }
    }

    struct TestMap {
        hash_builder: HashBuilder,
    }

    impl TestMap {
        fn hash<Q: ?Sized + Hash>(&self, key: &Q) -> usize {
            let mut h = self.hash_builder.build_hasher();
            key.hash(&mut h);
            h.finish() as usize
        }
    }

    let map = TestMap { hash_builder: HashBuilder };
    let key = "test_key";
    let expected_hash = map.hash(&key);
    assert_eq!(expected_hash, map.hash(&key));  // Check that hashing the same key gives the same result
}

#[test]
fn test_hash_with_integer_key() {
    use std::hash::{Hash, Hasher};
    use std::collections::hash_map::DefaultHasher;

    struct HashBuilder;

    impl HashBuilder {
        fn build_hasher(&self) -> DefaultHasher {
            DefaultHasher::new()
        }
    }

    struct TestMap {
        hash_builder: HashBuilder,
    }

    impl TestMap {
        fn hash<Q: ?Sized + Hash>(&self, key: &Q) -> usize {
            let mut h = self.hash_builder.build_hasher();
            key.hash(&mut h);
            h.finish() as usize
        }
    }

    let map = TestMap { hash_builder: HashBuilder };
    let key = 42;
    let expected_hash = map.hash(&key);
    assert_eq!(expected_hash, map.hash(&key));  // Check that hashing the same key gives the same result
}

