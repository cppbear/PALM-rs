// Answer 0

#[test]
fn test_from_hash_inserts_entry_correctly() {
    use core::hash::{BuildHasher, Hash};
    use hashbrown::hash_map::{HashMap, RawEntryMut};

    fn compute_hash<K: Hash + ?Sized, S: BuildHasher>(hash_builder: &S, key: &K) -> u64 {
        use core::hash::Hasher;
        let mut state = hash_builder.build_hasher();
        key.hash(&mut state);
        state.finish()
    }

    let mut map: HashMap<&str, u32> = HashMap::new();
    let key = "a";
    let hash = compute_hash(map.hasher(), &key);
    let entry: RawEntryMut<&str, u32, _> = map.raw_entry_mut().from_hash(hash, |k| k == &key);
    entry.insert(key, 100);
    assert_eq!(map[&"a"], 100);
}

#[test]
fn test_from_hash_no_insert_if_key_exists() {
    use core::hash::{BuildHasher, Hash};
    use hashbrown::hash_map::{HashMap, RawEntryMut};

    fn compute_hash<K: Hash + ?Sized, S: BuildHasher>(hash_builder: &S, key: &K) -> u64 {
        use core::hash::Hasher;
        let mut state = hash_builder.build_hasher();
        key.hash(&mut state);
        state.finish()
    }

    let mut map: HashMap<&str, u32> = HashMap::new();
    map.insert("a", 100);
    let key = "a";
    let hash = compute_hash(map.hasher(), &key);
    
    // This will not insert a new entry as the key already exists
    let entry: RawEntryMut<&str, u32, _> = map.raw_entry_mut().from_hash(hash, |k| k == &key);
    entry.insert(key, 200);
    
    // Verify that the existing value has not been changed
    assert_eq!(map[&"a"], 100);
}

#[test]
fn test_from_hash_with_non_existing_key_insert() {
    use core::hash::{BuildHasher, Hash};
    use hashbrown::hash_map::{HashMap, RawEntryMut};

    fn compute_hash<K: Hash + ?Sized, S: BuildHasher>(hash_builder: &S, key: &K) -> u64 {
        use core::hash::Hasher;
        let mut state = hash_builder.build_hasher();
        key.hash(&mut state);
        state.finish()
    }

    let mut map: HashMap<&str, u32> = HashMap::new();
    let key = "b";
    let hash = compute_hash(map.hasher(), &key);
    
    // This should insert a new entry as the key does not exist
    let entry: RawEntryMut<&str, u32, _> = map.raw_entry_mut().from_hash(hash, |k| k == &key);
    entry.insert(key, 200);
    
    // Verify that the value has been inserted
    assert_eq!(map[&"b"], 200);
}

