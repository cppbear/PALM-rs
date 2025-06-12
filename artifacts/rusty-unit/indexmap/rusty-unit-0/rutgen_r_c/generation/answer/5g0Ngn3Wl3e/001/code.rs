// Answer 0

#[test]
fn test_raw_entry_builder_mut_debug_fmt() {
    struct TestHasher; // A simple hasher for testing purposes
    impl BuildHasher for TestHasher {
        type Hasher = TestHasher;
        
        fn build_hasher(&self) -> Self::Hasher {
            TestHasher
        }
    }

    impl Hasher for TestHasher {
        fn write(&mut self, _bytes: &[u8]) {}
        fn finish(&self) -> u64 {
            0
        }
    }

    // Create a minimal IndexMap for testing
    struct IndexMapCore<K, V> {
        _marker: PhantomData<(K, V)>,
    }

    impl<K, V> IndexMap<K, V, TestHasher> {
        pub fn new() -> Self {
            IndexMap {
                core: IndexMapCore { _marker: PhantomData },
                hash_builder: TestHasher,
            }
        }
    }

    let mut index_map = IndexMap::new();
    let raw_entry_builder = RawEntryBuilderMut { map: &mut index_map };

    // Prepare formatter
    let mut buffer = String::new();
    let result = raw_entry_builder.fmt(&mut buffer);

    assert!(result.is_ok());
    assert!(buffer.contains("RawEntryBuilderMut"));
}

