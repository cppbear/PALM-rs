// Answer 0

#[test]
fn test_raw_entry_mut_from_hash_insert() {
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
fn test_raw_entry_mut_from_hash_no_match() {
    use core::hash::{BuildHasher, Hash};
    use hashbrown::hash_map::{HashMap, RawEntryMut};

    fn compute_hash<K: Hash + ?Sized, S: BuildHasher>(hash_builder: &S, key: &K) -> u64 {
        use core::hash::Hasher;
        let mut state = hash_builder.build_hasher();
        key.hash(&mut state);
        state.finish()
    }

    let mut map: HashMap<&str, u32> = HashMap::new();
    let key = "b"; // different key
    let hash = compute_hash(map.hasher(), &key);
    let entry: RawEntryMut<&str, u32, _> = map.raw_entry_mut().from_hash(hash, |k| k == &key);
    assert!(entry.is_none()); // should be none as key doesn't exist
}

#[test]
#[should_panic]
fn test_raw_entry_mut_from_hash_panic_on_invalid_key() {
    use core::hash::{BuildHasher, Hash};
    use hashbrown::hash_map::{HashMap, RawEntryMut};

    fn compute_hash<K: Hash + ?Sized, S: BuildHasher>(hash_builder: &S, key: &K) -> u64 {
        use core::hash::Hasher;
        let mut state = hash_builder.build_hasher();
        key.hash(&mut state);
        state.finish()
    }

    let mut map: HashMap<&str, u32> = HashMap::new();
    let key = "c";
    let hash = compute_hash(map.hasher(), &key);
    let entry: RawEntryMut<&str, u32, _> = map.raw_entry_mut().from_hash(hash, |k| k == &key);
    // Trying to insert a key that will panic (as 'c' isn't in the map)
    entry.insert("c", 200);
}

#[test]
fn test_raw_entry_mut_from_hash_multiple_entries() {
    use core::hash::{BuildHasher, Hash};
    use hashbrown::hash_map::{HashMap, RawEntryMut};

    fn compute_hash<K: Hash + ?Sized, S: BuildHasher>(hash_builder: &S, key: &K) -> u64 {
        use core::hash::Hasher;
        let mut state = hash_builder.build_hasher();
        key.hash(&mut state);
        state.finish()
    }

    let mut map: HashMap<&str, u32> = HashMap::new();
    let keys = ["d", "e"];
    
    for &key in &keys {
        let hash = compute_hash(map.hasher(), &key);
        let entry: RawEntryMut<&str, u32, _> = map.raw_entry_mut().from_hash(hash, |k| k == &key);
        entry.insert(key, 300);
    }
    
    assert_eq!(map[&"d"], 300);
    assert_eq!(map[&"e"], 300);
}

