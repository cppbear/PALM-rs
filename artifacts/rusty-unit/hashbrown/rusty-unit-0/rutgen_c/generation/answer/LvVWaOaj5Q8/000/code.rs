// Answer 0

#[test]
fn test_raw_entry_mut_from_hash_occupied() {
    use hashbrown::hash_map::{HashMap, RawEntryMut};
    use core::hash::{BuildHasher, Hash};

    fn compute_hash<K: Hash + ?Sized, S: BuildHasher>(hash_builder: &S, key: &K) -> u64 {
        use core::hash::Hasher;
        let mut state = hash_builder.build_hasher();
        key.hash(&mut state);
        state.finish()
    }

    struct DummyHasher;
    impl BuildHasher for DummyHasher {
        type Hasher = std::collections::hash_map::DefaultHasher;

        fn build_hasher(&self) -> Self::Hasher {
            std::collections::hash_map::DefaultHasher::new()
        }
    }

    let mut map: HashMap<&str, u32, DummyHasher> = HashMap::new();
    map.insert("a", 50);
    
    let key = "a";
    let hash = compute_hash(map.hasher(), &key);
    let entry: RawEntryMut<&str, u32, DummyHasher> = map.raw_entry_mut().from_hash(hash, |k| k == &key);

    if let RawEntryMut::Occupied(occupied) = entry {
        occupied.insert(key, 100);
    } else {
        panic!("Entry should be occupied");
    }
    
    assert_eq!(map[&"a"], 100);
}

#[test]
fn test_raw_entry_mut_from_hash_vacant() {
    use hashbrown::hash_map::{HashMap, RawEntryMut};
    use core::hash::{BuildHasher, Hash};

    fn compute_hash<K: Hash + ?Sized, S: BuildHasher>(hash_builder: &S, key: &K) -> u64 {
        use core::hash::Hasher;
        let mut state = hash_builder.build_hasher();
        key.hash(&mut state);
        state.finish()
    }

    struct DummyHasher;
    impl BuildHasher for DummyHasher {
        type Hasher = std::collections::hash_map::DefaultHasher;

        fn build_hasher(&self) -> Self::Hasher {
            std::collections::hash_map::DefaultHasher::new()
        }
    }

    let mut map: HashMap<&str, u32, DummyHasher> = HashMap::new();
    let key = "b";
    let hash = compute_hash(map.hasher(), &key);
    let entry: RawEntryMut<&str, u32, DummyHasher> = map.raw_entry_mut().from_hash(hash, |k| k == &key);

    if let RawEntryMut::Vacant(vacant) = entry {
        vacant.insert(key, 200);
    } else {
        panic!("Entry should be vacant");
    }
    
    assert_eq!(map[&"b"], 200);
}

