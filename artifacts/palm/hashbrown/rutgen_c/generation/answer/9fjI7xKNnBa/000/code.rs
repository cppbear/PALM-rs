// Answer 0

#[test]
fn test_raw_entry() {
    use core::hash::{BuildHasher, Hash};
    use crate::hashbrown::HashMap;

    let mut map: HashMap<&str, usize> = HashMap::new();
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

        assert_eq!(map.raw_entry().from_key(&k), kv);
        assert_eq!(map.raw_entry().from_hash(hash, |q| *q == k), kv);
        assert_eq!(map.raw_entry().from_key_hashed_nocheck(hash, &k), kv);
    }
}

#[test]
#[should_panic]
fn test_raw_entry_nonexistent_key() {
    use crate::hashbrown::HashMap;

    let mut map: HashMap<&str, usize> = HashMap::new();
    map.extend([("a", 100), ("b", 200)]);

    let kv = map.raw_entry().from_key("non_existent_key");
    assert!(kv.is_none());
}

