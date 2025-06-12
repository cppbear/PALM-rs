// Answer 0

#[cfg(test)]
fn test_insert_hashed_nocheck() {
    use core::hash::{BuildHasher, Hash, Hasher};
    use hashbrown::hash_map::{HashMap, RawEntryMut};
    use std::collections::hash_map::RandomState;

    struct SimpleHashBuilder;

    impl BuildHasher for SimpleHashBuilder {
        fn build_hasher(&self) -> RandomState {
            RandomState::new()
        }
    }

    let mut map: HashMap<&str, u32> = [("a", 100), ("b", 200)].into();
    let hash_builder = SimpleHashBuilder;

    let key = "c";
    let hash: u64 = {
        let mut hasher = hash_builder.build_hasher();
        key.hash(&mut hasher);
        hasher.finish()
    };

    match map.raw_entry_mut().from_key_hashed_nocheck(hash, &key) {
        RawEntryMut::Occupied(_) => panic!(),
        RawEntryMut::Vacant(v) => {
            let (k, v) = v.insert_hashed_nocheck(hash, key, 300);
            assert_eq!(*k, "c");
            assert_eq!(*v, 300);
        }
    }

    assert_eq!(map[&"c"], 300);
}

#[cfg(test)]
fn test_insert_hashed_nocheck_with_another_value() {
    use core::hash::{BuildHasher, Hash, Hasher};
    use hashbrown::hash_map::{HashMap, RawEntryMut};
    use std::collections::hash_map::RandomState;

    struct SimpleHashBuilder;

    impl BuildHasher for SimpleHashBuilder {
        fn build_hasher(&self) -> RandomState {
            RandomState::new()
        }
    }

    let mut map: HashMap<&str, u32> = [("x", 50), ("y", 150)].into();
    let hash_builder = SimpleHashBuilder;

    let key = "z";
    let hash: u64 = {
        let mut hasher = hash_builder.build_hasher();
        key.hash(&mut hasher);
        hasher.finish()
    };

    match map.raw_entry_mut().from_key_hashed_nocheck(hash, &key) {
        RawEntryMut::Occupied(_) => panic!(),
        RawEntryMut::Vacant(v) => {
            let (k, v) = v.insert_hashed_nocheck(hash, key, 450);
            assert_eq!(*k, "z");
            assert_eq!(*v, 450);
        }
    }

    assert_eq!(map[&"z"], 450);
}

