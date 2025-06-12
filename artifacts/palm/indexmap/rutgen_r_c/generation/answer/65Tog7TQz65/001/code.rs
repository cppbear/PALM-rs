// Answer 0

#[test]
fn test_get_index_of_existing_element() {
    struct TestHasher;
    impl BuildHasher for TestHasher {
        type Hasher = std::collections::hash_map::RandomState;
        fn build_hasher(&self) -> Self::Hasher {
            std::collections::hash_map::RandomState::new()
        }
    }
    
    let mut index_set = super::IndexSet::<i32, TestHasher> {
        map: super::IndexMap {
            core: super::IndexMapCore::new(),
            hash_builder: TestHasher,
        },
    };
    
    index_set.map.insert(1, ());
    assert_eq!(index_set.get_index_of(&1), Some(0));
}

#[test]
fn test_get_index_of_non_existent_element() {
    struct TestHasher;
    impl BuildHasher for TestHasher {
        type Hasher = std::collections::hash_map::RandomState;
        fn build_hasher(&self) -> Self::Hasher {
            std::collections::hash_map::RandomState::new()
        }
    }
    
    let mut index_set = super::IndexSet::<i32, TestHasher> {
        map: super::IndexMap {
            core: super::IndexMapCore::new(),
            hash_builder: TestHasher,
        },
    };
    
    index_set.map.insert(1, ());
    assert_eq!(index_set.get_index_of(&2), None);
}

#[test]
fn test_get_index_of_with_multiple_elements() {
    struct TestHasher;
    impl BuildHasher for TestHasher {
        type Hasher = std::collections::hash_map::RandomState;
        fn build_hasher(&self) -> Self::Hasher {
            std::collections::hash_map::RandomState::new()
        }
    }
    
    let mut index_set = super::IndexSet::<i32, TestHasher> {
        map: super::IndexMap {
            core: super::IndexMapCore::new(),
            hash_builder: TestHasher,
        },
    };
    
    index_set.map.insert(1, ());
    index_set.map.insert(2, ());
    index_set.map.insert(3, ());
    
    assert_eq!(index_set.get_index_of(&2), Some(1));
    assert_eq!(index_set.get_index_of(&3), Some(2));
}

#[test]
fn test_get_index_of_with_default_element() {
    struct TestHasher;
    impl BuildHasher for TestHasher {
        type Hasher = std::collections::hash_map::RandomState;
        fn build_hasher(&self) -> Self::Hasher {
            std::collections::hash_map::RandomState::new()
        }
    }
    
    let mut index_set = super::IndexSet::<i32, TestHasher> {
        map: super::IndexMap {
            core: super::IndexMapCore::new(),
            hash_builder: TestHasher,
        },
    };
    
    index_set.map.insert(0, ());
    assert_eq!(index_set.get_index_of(&0), Some(0));
}

#[test]
fn test_get_index_of_empty_set() {
    struct TestHasher;
    impl BuildHasher for TestHasher {
        type Hasher = std::collections::hash_map::RandomState;
        fn build_hasher(&self) -> Self::Hasher {
            std::collections::hash_map::RandomState::new()
        }
    }
    
    let index_set = super::IndexSet::<i32, TestHasher> {
        map: super::IndexMap {
            core: super::IndexMapCore::new(),
            hash_builder: TestHasher,
        },
    };
    
    assert_eq!(index_set.get_index_of(&1), None);
}

