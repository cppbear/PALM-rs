// Answer 0

#[test]
fn test_from_key_hashed_nocheck_existing_key() {
    use core::hash::{BuildHasher, Hash};
    use hashbrown::HashMap;

    fn compute_hash<K: Hash + ?Sized, S: BuildHasher>(hash_builder: &S, key: &K) -> u64 {
        use core::hash::Hasher;
        let mut state = hash_builder.build_hasher();
        key.hash(&mut state);
        state.finish()
    }

    let map: HashMap<&str, u32> = [("x", 300), ("y", 400)].into();
    let key = "x";
    let hash = compute_hash(map.hasher(), &key);
    assert_eq!(map.raw_entry().from_key_hashed_nocheck(hash, &key), Some((&"x", &300)));
}

#[test]
fn test_from_key_hashed_nocheck_non_existing_key() {
    use core::hash::{BuildHasher, Hash};
    use hashbrown::HashMap;

    fn compute_hash<K: Hash + ?Sized, S: BuildHasher>(hash_builder: &S, key: &K) -> u64 {
        use core::hash::Hasher;
        let mut state = hash_builder.build_hasher();
        key.hash(&mut state);
        state.finish()
    }

    let map: HashMap<&str, u32> = [("x", 300), ("y", 400)].into();
    let key = "z"; // This key does not exist in the map
    let hash = compute_hash(map.hasher(), &key);
    assert_eq!(map.raw_entry().from_key_hashed_nocheck(hash, &key), None);
}

#[test]
#[should_panic]
fn test_from_key_hashed_nocheck_panic_non_hashable_key() {
    use core::hash::{BuildHasher, Hash};
    use hashbrown::HashMap;

    struct NonHashable;

    let map: HashMap<&str, u32> = [("a", 100), ("b", 200)].into();
    let key = NonHashable; // Non-hashable type
    let hash = 0u64; // Dummy hash value
    // Expect to panic since NonHashable cannot be hashed
    let _ = map.raw_entry().from_key_hashed_nocheck(hash, &key);
}

