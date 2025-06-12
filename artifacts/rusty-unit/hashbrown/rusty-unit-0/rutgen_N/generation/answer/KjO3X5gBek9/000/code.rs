// Answer 0

#[test]
fn test_insert_hashed_nocheck_vacant() {
    use core::hash::{BuildHasher, Hash};
    use hashbrown::hash_map::{HashMap, RawEntryMut};
    use std::collections::hash_map::RandomState;

    let mut map: HashMap<&str, u32, RandomState> = [("a", 100), ("b", 200)].into();
    let key = "c";
    let hash = compute_hash(map.hasher(), &key);

    match map.raw_entry_mut().from_key_hashed_nocheck(hash, &key) {
        RawEntryMut::Occupied(_) => panic!(),
        RawEntryMut::Vacant(v) => {
            let (k, v) = v.insert_hashed_nocheck(hash, key, 300);
            assert_eq!(*k, "c");
            assert_eq!(*v, 300);
        },
    }

    assert_eq!(map[&"c"], 300);
}

fn compute_hash<K: Hash + ?Sized>(hash_builder: &RandomState, key: &K) -> u64 {
    use core::hash::Hasher;
    let mut state = hash_builder.build_hasher();
    key.hash(&mut state);
    state.finish()
}

