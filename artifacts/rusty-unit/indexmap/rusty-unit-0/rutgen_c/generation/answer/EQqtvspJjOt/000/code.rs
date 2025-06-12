// Answer 0

#[test]
fn test_raw_entry_builder_debug_fmt() {
    struct DummyHasher;

    impl BuildHasher for DummyHasher {
        type Hasher = std::hash::Hasher;

        fn build_hasher(&self) -> Self::Hasher {
            unimplemented!()
        }
    }

    let map: IndexMap<i32, i32, DummyHasher> = IndexMap {
        core: IndexMapCore::default(),
        hash_builder: DummyHasher,
    };
    
    let raw_entry_builder = RawEntryBuilder { map: &map };
    
    let mut output = Vec::new();
    {
        let mut formatter = fmt::Formatter::new(&mut output);
        raw_entry_builder.fmt(&mut formatter).unwrap();
    }
    
    let result = String::from_utf8(output).unwrap();
    assert!(result.contains("RawEntryBuilder"));
}

