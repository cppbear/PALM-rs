// Answer 0

#[test]
fn test_raw_entry_mut_v1() {
    struct SimpleHasher;
    
    impl BuildHasher for SimpleHasher {
        type Hasher = std::collections::hash_map::DefaultHasher;
        
        fn build_hasher(&self) -> Self::Hasher {
            std::collections::hash_map::DefaultHasher::new()
        }
    }

    let mut map: IndexMap<i32, i32, SimpleHasher> = IndexMap {
        core: IndexMapCore {
            indices: Indices::new(),
            entries: Entries::new(),
        },
        hash_builder: SimpleHasher,
    };
    
    let raw_entry_builder_mut = map.raw_entry_mut_v1();
    
    assert_eq!(raw_entry_builder_mut.map as *const _, &map as *const _);
}

#[test]
fn test_raw_entry_mut_v1_empty_map() {
    struct SimpleHasher;
    
    impl BuildHasher for SimpleHasher {
        type Hasher = std::collections::hash_map::DefaultHasher;
        
        fn build_hasher(&self) -> Self::Hasher {
            std::collections::hash_map::DefaultHasher::new()
        }
    }

    let mut map: IndexMap<u32, u32, SimpleHasher> = IndexMap {
        core: IndexMapCore {
            indices: Indices::new(),
            entries: Entries::new(),
        },
        hash_builder: SimpleHasher,
    };
    
    let raw_entry_builder_mut = map.raw_entry_mut_v1();
    
    assert_eq!(raw_entry_builder_mut.map as *const _, &map as *const _);
}

