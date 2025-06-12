// Answer 0

#[test]
fn test_difference_with_non_overlapping_sets() {
    struct TestHasher;

    impl BuildHasher for TestHasher {
        type Hasher = std::collections::hash_map::RandomState;

        fn build_hasher(&self) -> Self::Hasher {
            std::collections::hash_map::RandomState::new()
        }
    }

    let mut set_a: super::IndexSet<i32, TestHasher> = super::IndexSet {
        map: super::IndexMap {
            core: super::IndexMapCore::new(),
            hash_builder: TestHasher,
        },
    };

    let mut set_b: super::IndexSet<i32, TestHasher> = super::IndexSet {
        map: super::IndexMap {
            core: super::IndexMapCore::new(),
            hash_builder: TestHasher,
        },
    };

    set_a.insert(1);
    set_a.insert(2);
    set_a.insert(3);
    set_b.insert(4);
    set_b.insert(5);

    let difference: super::Difference<i32, TestHasher> = set_a.difference(&set_b);
    
    let results: Vec<_> = difference.iter.collect();
    assert_eq!(results, vec![1, 2, 3]);
}

#[test]
fn test_difference_with_identical_sets() {
    struct TestHasher;

    impl BuildHasher for TestHasher {
        type Hasher = std::collections::hash_map::RandomState;

        fn build_hasher(&self) -> Self::Hasher {
            std::collections::hash_map::RandomState::new()
        }
    }

    let mut set_a: super::IndexSet<i32, TestHasher> = super::IndexSet {
        map: super::IndexMap {
            core: super::IndexMapCore::new(),
            hash_builder: TestHasher,
        },
    };

    set_a.insert(1);
    set_a.insert(2);
    set_a.insert(3);
    
    let mut set_b: super::IndexSet<i32, TestHasher> = set_a.clone();

    let difference: super::Difference<i32, TestHasher> = set_a.difference(&set_b);
    
    let results: Vec<_> = difference.iter.collect();
    assert_eq!(results, Vec::<i32>::new());
}

#[test]
fn test_difference_with_empty_other_set() {
    struct TestHasher;

    impl BuildHasher for TestHasher {
        type Hasher = std::collections::hash_map::RandomState;

        fn build_hasher(&self) -> Self::Hasher {
            std::collections::hash_map::RandomState::new()
        }
    }

    let mut set_a: super::IndexSet<i32, TestHasher> = super::IndexSet {
        map: super::IndexMap {
            core: super::IndexMapCore::new(),
            hash_builder: TestHasher,
        },
    };

    set_a.insert(1);
    set_a.insert(2);

    let empty_set_b: super::IndexSet<i32, TestHasher> = super::IndexSet {
        map: super::IndexMap {
            core: super::IndexMapCore::new(),
            hash_builder: TestHasher,
        },
    };

    let difference: super::Difference<i32, TestHasher> = set_a.difference(&empty_set_b);
    
    let results: Vec<_> = difference.iter.collect();
    assert_eq!(results, vec![1, 2]);
}

#[test]
fn test_difference_with_large_sets() {
    struct TestHasher;

    impl BuildHasher for TestHasher {
        type Hasher = std::collections::hash_map::RandomState;

        fn build_hasher(&self) -> Self::Hasher {
            std::collections::hash_map::RandomState::new()
        }
    }

    let mut set_a: super::IndexSet<i32, TestHasher> = super::IndexSet {
        map: super::IndexMap {
            core: super::IndexMapCore::new(),
            hash_builder: TestHasher,
        },
    };

    let mut set_b: super::IndexSet<i32, TestHasher> = super::IndexSet {
        map: super::IndexMap {
            core: super::IndexMapCore::new(),
            hash_builder: TestHasher,
        },
    };

    for i in 0..1000 {
        set_a.insert(i);
    }

    for i in 500..1500 {
        set_b.insert(i);
    }

    let difference: super::Difference<i32, TestHasher> = set_a.difference(&set_b);
    
    let results: Vec<_> = difference.iter.collect();
    assert_eq!(results, (0..500).collect::<Vec<i32>>());
}

