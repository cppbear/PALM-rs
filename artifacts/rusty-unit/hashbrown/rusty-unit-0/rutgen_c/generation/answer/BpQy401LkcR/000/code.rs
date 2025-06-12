// Answer 0

#[test]
fn test_or_insert_with_vacant_entry() {
    use hashbrown::HashMap;

    struct TestHasher;
    impl BuildHasher for TestHasher {
        type Hasher = std::hash::BuildHasherDefault<std::hash::Hasher>; // Use any appropriate hasher

        fn build_hasher(&self) -> Self::Hasher {
            std::hash::BuildHasherDefault::default()
        }
    }

    let mut map: HashMap<String, u32, TestHasher> = HashMap::new();
    let entry_ref = map.entry_ref("key");
    let value = entry_ref.or_insert_with(|| 42);

    assert_eq!(*value, 42);
    assert_eq!(map["key"], 42);
}

#[test]
fn test_or_insert_with_occupied_entry() {
    use hashbrown::HashMap;

    struct TestHasher;
    impl BuildHasher for TestHasher {
        type Hasher = std::hash::BuildHasherDefault<std::hash::Hasher>; // Use any appropriate hasher

        fn build_hasher(&self) -> Self::Hasher {
            std::hash::BuildHasherDefault::default()
        }
    }

    let mut map: HashMap<String, u32, TestHasher> = HashMap::new();
    map.insert("key".to_string(), 10);
    let entry_ref = map.entry_ref("key");
    let value = entry_ref.or_insert_with(|| 20);

    assert_eq!(*value, 10);
    assert_eq!(map["key"], 10);
}

#[test]
fn test_or_insert_with_modify_entry() {
    use hashbrown::HashMap;

    struct TestHasher;
    impl BuildHasher for TestHasher {
        type Hasher = std::hash::BuildHasherDefault<std::hash::Hasher>; // Use any appropriate hasher

        fn build_hasher(&self) -> Self::Hasher {
            std::hash::BuildHasherDefault::default()
        }
    }

    let mut map: HashMap<String, u32, TestHasher> = HashMap::new();
    map.insert("key".to_string(), 10);
    let entry_ref = map.entry_ref("key");
    let value = entry_ref.or_insert_with(|| 20);
    *value *= 2;

    assert_eq!(*value, 20);
    assert_eq!(map["key"], 20);
}

#[test]
#[should_panic(expected = "insertion failed")]
fn test_or_insert_with_key_not_insertable() {
    use hashbrown::HashMap;
    use std::hash::BuildHasher;

    struct NonInsertable;

    struct TestHasher;
    impl BuildHasher for TestHasher {
        type Hasher = std::hash::BuildHasherDefault<std::hash::Hasher>; // Use any appropriate hasher

        fn build_hasher(&self) -> Self::Hasher {
            std::hash::BuildHasherDefault::default()
        }
    }

    let mut map: HashMap<NonInsertable, u32, TestHasher> = HashMap::new();
    let entry_ref = map.entry_ref(NonInsertable);
    entry_ref.or_insert_with(|| 42);
}

