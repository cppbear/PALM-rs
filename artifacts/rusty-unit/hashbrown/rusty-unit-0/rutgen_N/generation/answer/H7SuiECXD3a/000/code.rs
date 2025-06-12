// Answer 0

#[test]
fn test_from_key_hashed_nocheck_existing_key() {
    use core::hash::{BuildHasher, Hash};
    use hashbrown::HashMap;
    use hashbrown::raw_entry::RawEntryMut;

    fn compute_hash<K: Hash + ?Sized, S: BuildHasher>(hash_builder: &S, key: &K) -> u64 {
        use core::hash::Hasher;
        let mut state = hash_builder.build_hasher();
        key.hash(&mut state);
        state.finish()
    }

    let map: HashMap<&str, u32> = [("a", 100), ("b", 200)].into();
    let key = "a";
    let hash = compute_hash(map.hasher(), &key);
    assert_eq!(map.raw_entry().from_key_hashed_nocheck(hash, &key), Some((&"a", &100)));
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

    let map: HashMap<&str, u32> = [("a", 100), ("b", 200)].into();
    let key = "c";
    let hash = compute_hash(map.hasher(), &key);
    assert_eq!(map.raw_entry().from_key_hashed_nocheck(hash, &key), None);
}

#[test]
fn test_from_key_hashed_nocheck_boundary_conditions() {
    use core::hash::{BuildHasher, Hash};
    use hashbrown::HashMap;

    fn compute_hash<K: Hash + ?Sized, S: BuildHasher>(hash_builder: &S, key: &K) -> u64 {
        use core::hash::Hasher;
        let mut state = hash_builder.build_hasher();
        key.hash(&mut state);
        state.finish()
    }

    let map: HashMap<&str, u32> = [].into();
    let key = "a";
    let hash = compute_hash(map.hasher(), &key);
    assert_eq!(map.raw_entry().from_key_hashed_nocheck(hash, &key), None);
}

