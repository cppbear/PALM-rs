// Answer 0

#[test]
fn test_from_key_hashed_nocheck_occupied_entry() {
    use std::collections::hash_map::DefaultHasher;

    struct TestKey;
    
    impl Hash for TestKey {
        fn hash<H: Hasher>(&self, state: &mut H) {
            // Custom hash function for TestKey
            state.write_u64(42);
        }
    }
    
    impl Equivalent<TestKey> for TestKey {
        fn equivalent(&self, _: &TestKey) -> bool {
            true // Always true for simplification
        }
    }

    struct TestHasher;

    impl BuildHasher for TestHasher {
        type Hasher = DefaultHasher;
        fn build_hasher(&self) -> DefaultHasher {
            DefaultHasher::new()
        }
    }

    let mut map = IndexMap::<TestKey, i32, TestHasher> {
        core: IndexMapCore::new(),
        hash_builder: TestHasher,
    };
    
    let builder = RawEntryBuilderMut { map: &mut map };
    let entry = builder.from_key_hashed_nocheck(42, &TestKey);
    
    match entry {
        RawEntryMut::Occupied(_) => assert!(true), // Test passes if we get an occupied entry
        _ => assert!(false, "Expected an occupied entry"),
    }
}

#[test]
fn test_from_key_hashed_nocheck_vacant_entry() {
    use std::collections::hash_map::DefaultHasher;

    struct TestKey;

    impl Hash for TestKey {
        fn hash<H: Hasher>(&self, state: &mut H) {
            // Custom hash function for TestKey
            state.write_u64(84);
        }
    }

    impl Equivalent<TestKey> for TestKey {
        fn equivalent(&self, _: &TestKey) -> bool {
            false // Always false to simulate a vacant entry
        }
    }

    struct TestHasher;

    impl BuildHasher for TestHasher {
        type Hasher = DefaultHasher;
        fn build_hasher(&self) -> DefaultHasher {
            DefaultHasher::new()
        }
    }

    let mut map = IndexMap::<TestKey, i32, TestHasher> {
        core: IndexMapCore::new(),
        hash_builder: TestHasher,
    };

    let builder = RawEntryBuilderMut { map: &mut map };
    let entry = builder.from_key_hashed_nocheck(84, &TestKey);

    match entry {
        RawEntryMut::Vacant(_) => assert!(true), // Test passes if we get a vacant entry
        _ => assert!(false, "Expected a vacant entry"),
    }
}

