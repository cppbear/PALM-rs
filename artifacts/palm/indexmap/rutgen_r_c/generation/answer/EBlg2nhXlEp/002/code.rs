// Answer 0

#[test]
fn test_from_key_hashed_nocheck_found() {
    struct TestHasher;
    struct TestKey;
    struct TestValue;

    impl Hash for TestKey {
        fn hash<H: Hasher>(&self, _state: &mut H) {
            // hash implementation
        }
    }

    impl Equivalent<TestKey> for TestKey {
        fn equivalent(&self, _other: &TestKey) -> bool {
            true // always equal for testing
        }
    }

    let mut map: IndexMap<TestKey, TestValue, TestHasher> = IndexMap {
        core: IndexMapCore {
            indices: Indices {}, // mocked
            entries: Entries {}, // mocked
        },
        hash_builder: TestHasher,
    };

    let builder = RawEntryBuilder { map: &map };
    
    let hash = 123; // arbitrary hash value
    let key = TestKey;
    
    assert!(builder.from_key_hashed_nocheck(hash, &key).is_some());
}

#[test]
#[should_panic]
fn test_from_key_hashed_nocheck_not_found() {
    struct TestHasher;
    struct TestKey;
    struct TestValue;

    impl Hash for TestKey {
        fn hash<H: Hasher>(&self, _state: &mut H) {
            // hash implementation
        }
    }

    impl Equivalent<TestKey> for TestKey {
        fn equivalent(&self, _other: &TestKey) -> bool {
            false // always not equal for testing
        }
    }

    let mut map: IndexMap<TestKey, TestValue, TestHasher> = IndexMap {
        core: IndexMapCore {
            indices: Indices {}, // mocked
            entries: Entries {}, // mocked
        },
        hash_builder: TestHasher,
    };

    let builder = RawEntryBuilder { map: &map };
    
    let hash = 456; // another arbitrary hash value
    let key = TestKey;
    
    // This should panic as we do not expect to find the key in the map
    let _ = builder.from_key_hashed_nocheck(hash, &key).unwrap();
}

