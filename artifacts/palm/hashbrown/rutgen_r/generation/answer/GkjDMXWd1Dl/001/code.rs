// Answer 0

#[test]
fn test_from_hash_found_entry() {
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
    let entry = map.raw_entry().from_hash(hash, |k| k == &key);
    assert_eq!(entry, Some((&"a", &100)));
}

#[test]
fn test_from_hash_non_existent_entry() {
    use core::hash::{BuildHasher, Hash};
    use hashbrown::HashMap;

    fn compute_hash<K: Hash + ?Sized, S: BuildHasher>(hash_builder: &S, key: &K) -> u64 {
        use core::hash::Hasher;
        let mut state = hash_builder.build_hasher();
        key.hash(&mut state);
        state.finish()
    }

    let map: HashMap<&str, u32> = [("a", 100), ("b", 200)].into();
    let key = "c"; // Non-existent key
    let hash = compute_hash(map.hasher(), &key);
    let entry = map.raw_entry().from_hash(hash, |k| k == &key);
    assert_eq!(entry, None);
}

#[test]
#[should_panic]
fn test_from_hash_panic_condition() {
    use core::hash::{BuildHasher, Hash};
    use hashbrown::HashMap;

    fn compute_hash<K: Hash + ?Sized, S: BuildHasher>(hash_builder: &S, key: &K) -> u64 {
        use core::hash::Hasher;
        let mut state = hash_builder.build_hasher();
        key.hash(&mut state);
        state.finish()
    }

    let map: HashMap<&str, u32> = [("a", 100), ("b", 200)].into();
    let unhashable_key = (); // Using a key type that cannot be hashed
    let hash = compute_hash(map.hasher(), &unhashable_key); // Here we may encounter a panic
    let _ = map.raw_entry().from_hash(hash, |k| false);
}

