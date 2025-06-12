// Answer 0

#[test]
fn test_union_with_empty_sets() {
    struct DummyHasher;

    impl BuildHasher for DummyHasher {
        type Hasher = std::collections::hash_map::RandomState;

        fn build_hasher(&self) -> Self::Hasher {
            self
        }
    }
    
    let set1: IndexSet<i32, DummyHasher> = IndexSet { map: IndexMap { core: IndexMapCore::new(), hash_builder: DummyHasher } };
    let set2: IndexSet<i32, DummyHasher> = IndexSet { map: IndexMap { core: IndexMapCore::new(), hash_builder: DummyHasher } };
    
    let union_set = set1.union(&set2);
    
    assert_eq!(union_set.count(), 0);
}

#[test]
fn test_union_with_non_empty_sets() {
    struct DummyHasher;

    impl BuildHasher for DummyHasher {
        type Hasher = std::collections::hash_map::RandomState;

        fn build_hasher(&self) -> Self::Hasher {
            self
        }
    }
    
    let mut set1: IndexSet<i32, DummyHasher> = IndexSet { map: IndexMap { core: IndexMapCore::new(), hash_builder: DummyHasher } };
    set1.insert(1);
    set1.insert(2);

    let mut set2: IndexSet<i32, DummyHasher> = IndexSet { map: IndexMap { core: IndexMapCore::new(), hash_builder: DummyHasher } };
    set2.insert(2);
    set2.insert(3);
    
    let union_set = set1.union(&set2);
    
    let expected_values: Vec<i32> = vec![1, 2, 3];
    assert_eq!(union_set.collect::<Vec<_>>(), expected_values);
}

#[test]
fn test_union_with_one_empty_set() {
    struct DummyHasher;

    impl BuildHasher for DummyHasher {
        type Hasher = std::collections::hash_map::RandomState;

        fn build_hasher(&self) -> Self::Hasher {
            self
        }
    }
    
    let mut set1: IndexSet<i32, DummyHasher> = IndexSet { map: IndexMap { core: IndexMapCore::new(), hash_builder: DummyHasher } };
    set1.insert(1);
    
    let set2: IndexSet<i32, DummyHasher> = IndexSet { map: IndexMap { core: IndexMapCore::new(), hash_builder: DummyHasher } };
    
    let union_set = set1.union(&set2);
    
    let expected_values: Vec<i32> = vec![1];
    assert_eq!(union_set.collect::<Vec<_>>(), expected_values);
}

