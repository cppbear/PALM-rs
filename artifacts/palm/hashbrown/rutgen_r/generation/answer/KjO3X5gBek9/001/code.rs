// Answer 0

#[test]
fn test_insert_hashed_nocheck_vacant_entry() {
    use core::hash::{BuildHasher, Hash};
    use hashbrown::hash_map::{HashMap, RawEntryMut};
    use std::collections::hash_map::RandomState;

    let mut map: HashMap<&str, u32, RandomState> = [("a", 100), ("b", 200)].into();
    let key = "c";
    let hash = compute_hash(map.hasher(), &key);

    match map.raw_entry_mut().from_key_hashed_nocheck(hash, &key) {
        RawEntryMut::Occupied(_) => panic!("Expected a vacant entry, found occupied."),
        RawEntryMut::Vacant(v) => {
            let (k, v) = v.insert_hashed_nocheck(hash, key, 300);
            assert_eq!(*k, "c");
            assert_eq!(*v, 300);
        }
    }

    assert_eq!(map[&"c"], 300);
}

#[test]
#[should_panic]
fn test_insert_hashed_nocheck_occupied_entry() {
    use core::hash::{BuildHasher, Hash};
    use hashbrown::hash_map::{HashMap, RawEntryMut};
    use std::collections::hash_map::RandomState;

    let mut map: HashMap<&str, u32, RandomState> = [("c", 300)].into();
    let key = "c";
    let hash = compute_hash(map.hasher(), &key);

    match map.raw_entry_mut().from_key_hashed_nocheck(hash, &key) {
        RawEntryMut::Occupied(_) => {
            // This will execute, causing the panic
            panic!("Expected a vacant entry.");
        },
        RawEntryMut::Vacant(_) => (),
    }
}

fn compute_hash<K: Hash + ?Sized, S: BuildHasher>(hash_builder: &S, key: &K) -> u64 {
    use core::hash::Hasher;
    let mut state = hash_builder.build_hasher();
    key.hash(&mut state);
    state.finish()
}

