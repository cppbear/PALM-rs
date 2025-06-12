// Answer 0

#[test]
fn test_raw_entry_builder_mut_debug_fmt() {
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
    
    struct DummyIndexMap<K, V> {
        core: (),
        hash_builder: DummyHasher,
    }
    
    let mut map = DummyIndexMap {
        core: (),
        hash_builder: DummyHasher,
    };
    
    let raw_entry_builder = RawEntryBuilderMut {
        map: &mut map,
    };
    
    let mut buffer = String::new();
    let result = write!(buffer, "{:?}", raw_entry_builder);
    
    assert!(result.is_ok());
    assert!(buffer.contains("RawEntryBuilderMut"));
}

