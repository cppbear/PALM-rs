// Answer 0

#[test]
fn test_raw_entry_builder_debug_fmt() {
    struct DummyHasher;

    impl BuildHasher for DummyHasher {
        type Hasher = DummyHasher;

        fn build_hasher(&self) -> Self::Hasher {
            DummyHasher
        }
    }

    impl Hasher for DummyHasher {
        fn write(&mut self, _: &[u8]) {}
        fn finish(&self) -> u64 {
            0
        }
    }

    struct MockIndexMap;

    impl fmt::Debug for MockIndexMap {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("MockIndexMap").finish()
        }
    }

    let mock_map = MockIndexMap;
    let raw_entry_builder = RawEntryBuilder {
        map: &mock_map,
    };

    let result = format!("{:?}", raw_entry_builder);
    assert_eq!(result, "RawEntryBuilder { /* fields omitted */ }");
}

