// Answer 0

#[test]
fn test_raw_entry_from_key_hashed_nocheck() {
    use core::hash::{BuildHasher, Hash};
    use hashbrown::hash_map::{HashMap, RawEntryMut};
    
    struct SimpleHasher;
    
    impl BuildHasher for SimpleHasher {
        type Hasher = std::collections::hash_map::RandomState;

        fn build_hasher(&self) -> Self::Hasher {
            std::collections::hash_map::RandomState::new()
        }
    }
    
    let mut map: HashMap<&str, u32, SimpleHasher> = HashMap::new();
    let key = "test_key";
    let hash = compute_hash(&SimpleHasher, &key);
    
    let mut entry: RawEntryMut<&str, u32, SimpleHasher> = map.raw_entry_mut().from_key_hashed_nocheck(hash, &key);
    entry.insert(key, 42);
    
    assert_eq!(map[&"test_key"], 42);
}

fn compute_hash<K: Hash + ?Sized, S: BuildHasher>(hash_builder: &S, key: &K) -> u64 {
    use core::hash::Hasher;
    let mut state = hash_builder.build_hasher();
    key.hash(&mut state);
    state.finish()
}

