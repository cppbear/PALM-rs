// Answer 0

#[test]
fn test_from_hash_occupied_entry() {
    use hashbrown::hash_map::{HashMap, RawEntryMut};
    use core::hash::{BuildHasher, Hash};

    // Create a simple hash map
    let mut map: HashMap<&str, u32> = HashMap::new();
    let key = "a";
    map.insert(key, 50);

    // Function to compute hash
    fn compute_hash<K: Hash + ?Sized, S: BuildHasher>(hash_builder: &S, key: &K) -> u64 {
        use core::hash::Hasher;
        let mut state = hash_builder.build_hasher();
        key.hash(&mut state);
        state.finish()
    }

    // Compute the hash for the key
    let hash = compute_hash(map.hasher(), &key);
    
    // Get the RawEntryMut using the computed hash and a matching closure
    let entry: RawEntryMut<&str, u32, _> = map.raw_entry_mut().from_hash(hash, |k| k == &key);
    
    // Since entry should be occupied, we can assert its presence
    match entry {
        RawEntryMut::Occupied(_) => {}
        RawEntryMut::Vacant(_) => panic!("Expected occupied entry"),
    }
}

#[test]
fn test_from_hash_vacant_entry() {
    use hashbrown::hash_map::{HashMap, RawEntryMut};
    use core::hash::{BuildHasher, Hash};

    // Create a simple hash map
    let mut map: HashMap<&str, u32> = HashMap::new();
    let key = "b";
    
    // Function to compute hash
    fn compute_hash<K: Hash + ?Sized, S: BuildHasher>(hash_builder: &S, key: &K) -> u64 {
        use core::hash::Hasher;
        let mut state = hash_builder.build_hasher();
        key.hash(&mut state);
        state.finish()
    }

    // Compute the hash for a non-existing key
    let hash = compute_hash(map.hasher(), &key);
    
    // Get the RawEntryMut using the computed hash and a matching closure
    let entry: RawEntryMut<&str, u32, _> = map.raw_entry_mut().from_hash(hash, |k| k == &key);
    
    // Since entry should be vacant, we can assert its presence
    match entry {
        RawEntryMut::Occupied(_) => panic!("Expected vacant entry"),
        RawEntryMut::Vacant(_) => {}
    }
}

