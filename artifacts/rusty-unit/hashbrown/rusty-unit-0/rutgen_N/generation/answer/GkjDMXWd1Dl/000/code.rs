// Answer 0

#[test]
fn test_from_hash_existing_entry() {
    use core::hash::{BuildHasher, Hash};
    use hashbrown::HashMap;

    fn compute_hash<K: Hash + ?Sized, S: BuildHasher>(hash_builder: &S, key: &K) -> u64 {
        use core::hash::Hasher;
        let mut state = hash_builder.build_hasher();
        key.hash(&mut state);
        state.finish()
    }

    let map: HashMap<&str, u32> = [("a", 100), ("b", 200)].into();
    let key = "a";
    let hash = compute_hash(map.hasher(), &key);
    assert_eq!(map.raw_entry().from_hash(hash, |k| k == &key), Some((&"a", &100)));
}

#[test]
fn test_from_hash_non_existing_entry() {
    use core::hash::{BuildHasher, Hash};
    use hashbrown::HashMap;

    fn compute_hash<K: Hash + ?Sized, S: BuildHasher>(hash_builder: &S, key: &K) -> u64 {
        use core::hash::Hasher;
        let mut state = hash_builder.build_hasher();
        key.hash(&mut state);
        state.finish()
    }

    let map: HashMap<&str, u32> = [("a", 100), ("b", 200)].into();
    let key = "c";
    let hash = compute_hash(map.hasher(), &key);
    assert_eq!(map.raw_entry().from_hash(hash, |k| k == &key), None);
}

#[test]
fn test_from_hash_partial_match() {
    use core::hash::{BuildHasher, Hash};
    use hashbrown::HashMap;

    fn compute_hash<K: Hash + ?Sized, S: BuildHasher>(hash_builder: &S, key: &K) -> u64 {
        use core::hash::Hasher;
        let mut state = hash_builder.build_hasher();
        key.hash(&mut state);
        state.finish()
    }

    let map: HashMap<&str, u32> = [("key1", 300), ("key2", 400)].into();
    let key = "key3"; // not in map
    let hash = compute_hash(map.hasher(), &key);
    assert_eq!(map.raw_entry().from_hash(hash, |k| k == &key), None);
}

