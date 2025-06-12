// Answer 0

#[test]
fn test_raw_entry_from_key() {
    use core::hash::{BuildHasher, Hash};
    use hashbrown::HashMap;

    let mut map = HashMap::new();
    map.extend([("a", 100), ("b", 200), ("c", 300)]);

    fn compute_hash<K: Hash + ?Sized, S: BuildHasher>(hash_builder: &S, key: &K) -> u64 {
        use core::hash::Hasher;
        let mut state = hash_builder.build_hasher();
        key.hash(&mut state);
        state.finish()
    }

    for k in ["a", "b", "c", "d", "e", "f"] {
        let v = map.get(&k).cloned();
        let kv = v.as_ref().map(|v| (&k, v));

        assert_eq!(map.raw_entry().from_key(&k), kv);
    }
}

#[test]
fn test_raw_entry_from_hash() {
    use core::hash::{BuildHasher, Hash};
    use hashbrown::HashMap;

    let mut map = HashMap::new();
    map.extend([("a", 100), ("b", 200), ("c", 300)]);

    fn compute_hash<K: Hash + ?Sized, S: BuildHasher>(hash_builder: &S, key: &K) -> u64 {
        use core::hash::Hasher;
        let mut state = hash_builder.build_hasher();
        key.hash(&mut state);
        state.finish()
    }

    for k in ["a", "b", "c", "d", "e", "f"] {
        let hash = compute_hash(map.hasher(), k);
        let v = map.get(&k).cloned();
        let kv = v.as_ref().map(|v| (&k, v));

        assert_eq!(map.raw_entry().from_hash(hash, |q| *q == k), kv);
    }
}

#[test]
fn test_raw_entry_from_key_hashed_nocheck() {
    use core::hash::{BuildHasher, Hash};
    use hashbrown::HashMap;

    let mut map = HashMap::new();
    map.extend([("a", 100), ("b", 200), ("c", 300)]);

    fn compute_hash<K: Hash + ?Sized, S: BuildHasher>(hash_builder: &S, key: &K) -> u64 {
        use core::hash::Hasher;
        let mut state = hash_builder.build_hasher();
        key.hash(&mut state);
        state.finish()
    }

    for k in ["a", "b", "c", "d", "e", "f"] {
        let hash = compute_hash(map.hasher(), k);
        let v = map.get(&k).cloned();
        let kv = v.as_ref().map(|v| (&k, v));

        assert_eq!(map.raw_entry().from_key_hashed_nocheck(hash, &k), kv);
    }
}

