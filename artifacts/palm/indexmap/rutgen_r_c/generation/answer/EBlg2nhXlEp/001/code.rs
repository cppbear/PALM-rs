// Answer 0

#[test]
fn test_from_key_hashed_nocheck_none() {
    struct TestHasher;

    impl BuildHasher for TestHasher {
        type Hasher = std::collections::hash_map::DefaultHasher;

        fn build_hasher(&self) -> Self::Hasher {
            std::collections::hash_map::DefaultHasher::new()
        }
    }

    struct DummyKey;

    impl Equivalent<DummyKey> for DummyKey {
        fn equivalent(&self, _: &DummyKey) -> bool {
            false // always returns false to induce None
        }
    }

    let mut map = IndexMap {
        core: IndexMapCore {
            indices: Indices::default(),
            entries: Entries::default(),
        },
        hash_builder: TestHasher,
    };

    let builder = RawEntryBuilder { map: &map };
    let result = builder.from_key_hashed_nocheck(42, &DummyKey);

    assert!(result.is_none()); // Expecting None since equivalent condition is not met.
}

#[test]
fn test_from_key_hashed_nocheck_key_not_found() {
    struct TestHasher;

    impl BuildHasher for TestHasher {
        type Hasher = std::collections::hash_map::DefaultHasher;

        fn build_hasher(&self) -> Self::Hasher {
            std::collections::hash_map::DefaultHasher::new()
        }
    }

    let mut map = IndexMap {
        core: IndexMapCore {
            indices: Indices::default(),
            entries: Entries::default(),
        },
        hash_builder: TestHasher,
    };

    let builder = RawEntryBuilder { map: &map };
    let result = builder.from_key_hashed_nocheck(101, &DummyKey);

    assert!(result.is_none()); // Expecting None since index lookup will fail.
}

