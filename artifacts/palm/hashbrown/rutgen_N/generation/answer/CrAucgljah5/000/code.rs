// Answer 0

#[test]
fn test_raw_entry_mut_existing_key_insert_and_update() {
    use core::hash::{BuildHasher, Hash};
    use hashbrown::hash_map::{HashMap, RawEntryMut};

    let mut map = HashMap::new();
    map.extend([("a", 100), ("b", 200), ("c", 300)]);

    fn compute_hash<K: Hash + ?Sized, S: BuildHasher>(hash_builder: &S, key: &K) -> u64 {
        use core::hash::Hasher;
        let mut state = hash_builder.build_hasher();
        key.hash(&mut state);
        state.finish()
    }

    // Existing key (insert and update)
    match map.raw_entry_mut().from_key(&"a") {
        RawEntryMut::Vacant(_) => unreachable!(),
        RawEntryMut::Occupied(mut view) => {
            assert_eq!(view.get(), &100);
            let v = view.get_mut();
            let new_v = (*v) * 10;
            *v = new_v;
            assert_eq!(view.insert(1111), 1000);
        }
    }

    assert_eq!(map[&"a"], 1111);
    assert_eq!(map.len(), 3);
}

#[test]
fn test_raw_entry_mut_existing_key_take() {
    use core::hash::{BuildHasher, Hash};
    use hashbrown::hash_map::{HashMap, RawEntryMut};

    let mut map = HashMap::new();
    map.extend([("a", 100), ("b", 200), ("c", 300)]);

    fn compute_hash<K: Hash + ?Sized, S: BuildHasher>(hash_builder: &S, key: &K) -> u64 {
        use core::hash::Hasher;
        let mut state = hash_builder.build_hasher();
        key.hash(&mut state);
        state.finish()
    }

    // Existing key (take)
    let hash = compute_hash(map.hasher(), &"c");
    match map.raw_entry_mut().from_key_hashed_nocheck(hash, &"c") {
        RawEntryMut::Vacant(_) => unreachable!(),
        RawEntryMut::Occupied(view) => {
            assert_eq!(view.remove_entry(), ("c", 300));
        }
    }
    assert_eq!(map.raw_entry().from_key(&"c"), None);
    assert_eq!(map.len(), 2);
}

#[test]
fn test_raw_entry_mut_nonexistent_key_insert_and_update() {
    use core::hash::{BuildHasher, Hash};
    use hashbrown::hash_map::{HashMap, RawEntryMut};

    let mut map = HashMap::new();
    map.extend([("a", 100), ("b", 200), ("c", 300)]);

    fn compute_hash<K: Hash + ?Sized, S: BuildHasher>(hash_builder: &S, key: &K) -> u64 {
        use core::hash::Hasher;
        let mut state = hash_builder.build_hasher();
        key.hash(&mut state);
        state.finish()
    }

    // Nonexistent key (insert and update)
    let key = "d";
    let hash = compute_hash(map.hasher(), &key);
    match map.raw_entry_mut().from_hash(hash, |q| *q == key) {
        RawEntryMut::Occupied(_) => unreachable!(),
        RawEntryMut::Vacant(view) => {
            let (k, value) = view.insert("d", 4000);
            assert_eq!((*k, *value), ("d", 4000));
            *value = 40000;
        }
    }
    assert_eq!(map[&"d"], 40000);
    assert_eq!(map.len(), 3);
}

#[test]
fn test_raw_entry_mut_nonexistent_key_take() {
    use core::hash::{BuildHasher, Hash};
    use hashbrown::hash_map::{HashMap, RawEntryMut};

    let mut map = HashMap::new();
    map.extend([("a", 100), ("b", 200), ("c", 300)]);

    fn compute_hash<K: Hash + ?Sized, S: BuildHasher>(hash_builder: &S, key: &K) -> u64 {
        use core::hash::Hasher;
        let mut state = hash_builder.build_hasher();
        key.hash(&mut state);
        state.finish()
    }

    // Nonexistent key (take)
    let key = "d";
    let hash = compute_hash(map.hasher(), &key);
    match map.raw_entry_mut().from_hash(hash, |q| *q == key) {
        RawEntryMut::Vacant(_) => unreachable!(),
        RawEntryMut::Occupied(view) => {
            assert_eq!(view.remove_entry(), ("d", 40000));
        }
    }
    assert_eq!(map.get(&"d"), None);
    assert_eq!(map.len(), 2);
}

