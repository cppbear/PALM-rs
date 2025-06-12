// Answer 0

#[test]
fn test_raw_entry_mut_vacant_entry() {
    use hashbrown::hash_map::RawEntryMut;
    use hashbrown::HashMap;
    use core::hash::{BuildHasher, Hash};

    struct SimpleHasher;
    impl BuildHasher for SimpleHasher {
        type Hasher = std::hash::StdHasher;

        fn build_hasher(&self) -> Self::Hasher {
            std::hash::StdHasher::new()
        }
    }

    let mut map: HashMap<&str, i32, SimpleHasher> = HashMap::new();
    let key = "a";
    let hash = SimpleHasher::default().build_hasher().finish();
    let entry: RawEntryMut<&str, i32, SimpleHasher> = map.raw_entry_mut().from_key_hashed_nocheck(hash, &key);

    match entry {
        RawEntryMut::Vacant(vacant_entry) => {
            vacant_entry.insert(key, 42);
        },
        RawEntryMut::Occupied(_) => unreachable!(),
    }

    assert_eq!(map[&key], 42);
}

#[test]
fn test_raw_entry_mut_occupied_entry() {
    use hashbrown::hash_map::RawEntryMut;
    use hashbrown::HashMap;
    use core::hash::{BuildHasher, Hash};

    struct SimpleHasher;
    impl BuildHasher for SimpleHasher {
        type Hasher = std::hash::StdHasher;

        fn build_hasher(&self) -> Self::Hasher {
            std::hash::StdHasher::new()
        }
    }

    let mut map: HashMap<&str, i32, SimpleHasher> = HashMap::new();
    map.insert("a", 100);
    let key = "a";
    let hash = SimpleHasher::default().build_hasher().finish();
    let entry: RawEntryMut<&str, i32, SimpleHasher> = map.raw_entry_mut().from_key_hashed_nocheck(hash, &key);

    match entry {
        RawEntryMut::Occupied(occupied_entry) => {
            assert_eq!(occupied_entry.get(), &100);
        },
        RawEntryMut::Vacant(_) => unreachable!(),
    }
}

#[test]
fn test_raw_entry_mut_non_existent_key() {
    use hashbrown::hash_map::RawEntryMut;
    use hashbrown::HashMap;
    use core::hash::{BuildHasher, Hash};

    struct SimpleHasher;
    impl BuildHasher for SimpleHasher {
        type Hasher = std::hash::StdHasher;

        fn build_hasher(&self) -> Self::Hasher {
            std::hash::StdHasher::new()
        }
    }

    let mut map: HashMap<&str, i32, SimpleHasher> = HashMap::new();
    let key = "non_existent_key";
    let hash = SimpleHasher::default().build_hasher().finish();
    let entry: RawEntryMut<&str, i32, SimpleHasher> = map.raw_entry_mut().from_key_hashed_nocheck(hash, &key);

    match entry {
        RawEntryMut::Vacant(vacant_entry) => {
            vacant_entry.insert(key, 100);
        },
        RawEntryMut::Occupied(_) => unreachable!(),
    }

    assert_eq!(map[key], 100);
}

