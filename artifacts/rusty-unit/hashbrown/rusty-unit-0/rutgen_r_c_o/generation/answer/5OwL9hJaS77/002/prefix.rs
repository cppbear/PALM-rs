// Answer 0

#[test]
fn test_or_insert_with_occupied_entry() {
    use hashbrown::hash_map::{RawEntryMut, RawOccupiedEntryMut};
    use hashbrown::HashMap;
    use core::hash::BuildHasher;

    struct MyHasher;

    impl BuildHasher for MyHasher {
        type Hasher = std::hash::Hasher;

        fn build_hasher(&self) -> Self::Hasher {
            std::collections::hash_map::DefaultHasher::new()
        }
    }

    let mut map: HashMap<&str, i32, MyHasher> = HashMap::new();
    map.insert("key1", 42);

    let mut entry = match map.raw_entry_mut().from_key("key1") {
        RawEntryMut::Occupied(entry) => entry,
        RawEntryMut::Vacant(_) => unreachable!(),
    };

    let (key_ref, value_ref) = entry.or_insert("key1", 100);
    // Using key_ref and value_ref to simulate further usage
}

#[test]
fn test_or_insert_with_another_occupied_entry() {
    use hashbrown::hash_map::{RawEntryMut, RawOccupiedEntryMut};
    use hashbrown::HashMap;
    use core::hash::BuildHasher;

    struct MyHasher;

    impl BuildHasher for MyHasher {
        type Hasher = std::hash::Hasher;

        fn build_hasher(&self) -> Self::Hasher {
            std::collections::hash_map::DefaultHasher::new()
        }
    }

    let mut map: HashMap<i32, String, MyHasher> = HashMap::new();
    map.insert(1, "one".to_string());

    let mut entry = match map.raw_entry_mut().from_key(&1) {
        RawEntryMut::Occupied(entry) => entry,
        RawEntryMut::Vacant(_) => unreachable!(),
    };

    let (key_ref, value_ref) = entry.or_insert(1, "default".to_string());
    // Using key_ref and value_ref to simulate further usage
}

