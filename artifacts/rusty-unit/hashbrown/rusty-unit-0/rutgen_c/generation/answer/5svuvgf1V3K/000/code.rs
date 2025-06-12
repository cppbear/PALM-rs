// Answer 0

#[test]
fn test_insert_entry() {
    use crate::HashMap;
    use crate::raw::Global;
    use std::hash::BuildHasherDefault;

    struct TestHasher;
    impl BuildHasher for TestHasher {
        type Hasher = std::collections::hash_map::RandomState;

        fn build_hasher(&self) -> Self::Hasher {
            std::collections::hash_map::RandomState::new()
        }
    }

    let mut map: HashMap<&str, u32, TestHasher, Global> = HashMap::new();

    let key = "test_key";
    let value = 42;

    let vacant_entry = VacantEntry {
        hash: 0, // Will be calculated in the actual implementation
        key,
        table: &mut map,
    };

    let occupied_entry = vacant_entry.insert_entry(value);

    // Check if the entry was inserted correctly
    assert_eq!(map.get(key), Some(&value));
    assert_eq!(occupied_entry.elem, map.table.table.get(&key).unwrap());
}

#[test]
#[should_panic]
fn test_insert_entry_panic_on_double_insert() {
    use crate::HashMap;
    use crate::raw::Global;
    use std::hash::BuildHasherDefault;

    struct TestHasher;
    impl BuildHasher for TestHasher {
        type Hasher = std::collections::hash_map::RandomState;

        fn build_hasher(&self) -> Self::Hasher {
            std::collections::hash_map::RandomState::new()
        }
    }

    let mut map: HashMap<&str, u32, TestHasher, Global> = HashMap::new();

    let key = "panic_key";

    let vacant_entry = VacantEntry {
        hash: 0, // Will be calculated in the actual implementation
        key,
        table: &mut map,
    };

    // First insert
    vacant_entry.insert_entry(99);

    // Attempting to insert again with the same key should panic
    vacant_entry.insert_entry(100);
}

