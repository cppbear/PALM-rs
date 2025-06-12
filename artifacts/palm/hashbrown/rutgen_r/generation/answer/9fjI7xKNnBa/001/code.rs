// Answer 0

#[test]
fn test_raw_entry_builder_with_existing_keys() {
    use core::hash::{BuildHasher, Hash};
    use hashbrown::HashMap;

    struct SimpleHasher;

    impl BuildHasher for SimpleHasher {
        type Hasher = std::collections::hash_map::DefaultHasher;

        fn build_hasher(&self) -> Self::Hasher {
            std::collections::hash_map::DefaultHasher::new()
        }
    }

    let mut map: HashMap<&str, i32, SimpleHasher> = HashMap::new();
    map.extend([("a", 100), ("b", 200), ("c", 300)]);

    fn compute_hash<K: Hash + ?Sized, S: BuildHasher>(hash_builder: &S, key: &K) -> u64 {
        use core::hash::Hasher;
        let mut state = hash_builder.build_hasher();
        key.hash(&mut state);
        state.finish()
    }

    for k in ["a", "b", "c"] {
        let hash = compute_hash(map.hasher(), k);
        let v = map.get(&k).cloned();
        let kv = v.as_ref().map(|v| (&k, v));

        let raw_entry = map.raw_entry();
        assert_eq!(raw_entry.from_key(&k), kv);
        assert_eq!(raw_entry.from_hash(hash, |q| *q == k), kv);
        assert_eq!(raw_entry.from_key_hashed_nocheck(hash, &k), kv);
    }
}

#[test]
fn test_raw_entry_builder_with_non_existing_keys() {
    use core::hash::{BuildHasher, Hash};
    use hashbrown::HashMap;

    struct SimpleHasher;

    impl BuildHasher for SimpleHasher {
        type Hasher = std::collections::hash_map::DefaultHasher;

        fn build_hasher(&self) -> Self::Hasher {
            std::collections::hash_map::DefaultHasher::new()
        }
    }

    let mut map: HashMap<&str, i32, SimpleHasher> = HashMap::new();
    map.extend([("a", 100), ("b", 200), ("c", 300)]);

    fn compute_hash<K: Hash + ?Sized, S: BuildHasher>(hash_builder: &S, key: &K) -> u64 {
        use core::hash::Hasher;
        let mut state = hash_builder.build_hasher();
        key.hash(&mut state);
        state.finish()
    }

    for k in ["d", "e", "f"] {
        let hash = compute_hash(map.hasher(), k);
        let kv: Option<(&str, &i32)> = None;

        let raw_entry = map.raw_entry();
        assert_eq!(raw_entry.from_key(&k), kv);
        assert_eq!(raw_entry.from_hash(hash, |q| *q == k), kv);
        assert_eq!(raw_entry.from_key_hashed_nocheck(hash, &k), kv);
    }
}

