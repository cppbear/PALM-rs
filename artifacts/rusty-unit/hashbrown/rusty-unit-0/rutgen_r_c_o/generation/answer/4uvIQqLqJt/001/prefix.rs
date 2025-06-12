// Answer 0

#[test]
fn test_get_from_vacant_entry() {
    use hashbrown::hash_set::{Entry, HashSet};
    use std::hash::BuildHasherDefault;
    use std::hash::Hash;

    struct MyHasher;

    impl BuildHasher for MyHasher {
        type Hasher = std::collections::hash_map::DefaultHasher;

        fn build_hasher(&self) -> Self::Hasher {
            std::collections::hash_map::DefaultHasher::new()
        }
    }

    let mut set: HashSet<&str, BuildHasherDefault<MyHasher>> = HashSet::new();
    let entry: Entry<&str, BuildHasherDefault<MyHasher>> = set.entry("poneyland");
    match entry {
        Entry::Vacant(vacant) => {
            let key = vacant.get();
            let new_entry = vacant.insert();
            let value = new_entry.get();
        }
        _ => unreachable!(),
    }
}

#[test]
fn test_get_from_existing_vacant_entry() {
    use hashbrown::hash_set::{Entry, HashSet};
    use std::hash::BuildHasherDefault;

    struct MyHasher;

    impl std::hash::BuildHasher for MyHasher {
        type Hasher = std::collections::hash_map::DefaultHasher;

        fn build_hasher(&self) -> Self::Hasher {
            std::collections::hash_map::DefaultHasher::new()
        }
    }

    let mut set: HashSet<&str, BuildHasherDefault<MyHasher>> = HashSet::new();
    let entry: Entry<&str, BuildHasherDefault<MyHasher>> = set.entry("horseland");
    match entry {
        Entry::Vacant(vacant) => {
            let key = vacant.get();
            let new_entry = vacant.insert();
            let value = new_entry.get();
        }
        _ => unreachable!(),
    }
}

