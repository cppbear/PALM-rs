// Answer 0

#[test]
fn test_union_with_empty_sets() {
    struct HashOnly;

    use std::collections::hash_map::RandomState;

    let set1: IndexSet<i32, RandomState> = IndexSet {
        map: IndexMap {
            core: IndexMapCore::new(),
            hash_builder: RandomState::new(),
        },
    };
    let set2: IndexSet<i32, RandomState> = IndexSet {
        map: IndexMap {
            core: IndexMapCore::new(),
            hash_builder: RandomState::new(),
        },
    };

    let result = set1.union(&set2);
    assert_eq!(result.iter().count(), 0);
}

#[test]
fn test_union_with_non_empty_set_and_empty_set() {
    struct HashOnly;

    use std::collections::hash_map::RandomState;

    let mut set1: IndexSet<i32, RandomState> = IndexSet {
        map: IndexMap {
            core: IndexMapCore::new(),
            hash_builder: RandomState::new(),
        },
    };
    set1.insert(1);
    set1.insert(2);

    let set2: IndexSet<i32, RandomState> = IndexSet {
        map: IndexMap {
            core: IndexMapCore::new(),
            hash_builder: RandomState::new(),
        },
    };

    let result = set1.union(&set2);
    assert_eq!(result.iter().collect::<Vec<_>>(), vec![&1, &2]);
}

#[test]
fn test_union_with_non_empty_sets() {
    struct HashOnly;

    use std::collections::hash_map::RandomState;

    let mut set1: IndexSet<i32, RandomState> = IndexSet {
        map: IndexMap {
            core: IndexMapCore::new(),
            hash_builder: RandomState::new(),
        },
    };
    set1.insert(1);
    set1.insert(2);

    let mut set2: IndexSet<i32, RandomState> = IndexSet {
        map: IndexMap {
            core: IndexMapCore::new(),
            hash_builder: RandomState::new(),
        },
    };
    set2.insert(2);
    set2.insert(3);

    let result = set1.union(&set2);
    assert_eq!(result.iter().collect::<Vec<_>>(), vec![&1, &2, &3]);
}

#[test]
fn test_union_with_identical_sets() {
    struct HashOnly;

    use std::collections::hash_map::RandomState;

    let mut set1: IndexSet<i32, RandomState> = IndexSet {
        map: IndexMap {
            core: IndexMapCore::new(),
            hash_builder: RandomState::new(),
        },
    };
    set1.insert(1);
    set1.insert(2);

    let mut set2: IndexSet<i32, RandomState> = IndexSet {
        map: IndexMap {
            core: IndexMapCore::new(),
            hash_builder: RandomState::new(),
        },
    };
    set2.insert(1);
    set2.insert(2);

    let result = set1.union(&set2);
    assert_eq!(result.iter().collect::<Vec<_>>(), vec![&1, &2]);
}

#[test]
fn test_union_with_sets_of_different_sizes() {
    struct HashOnly;

    use std::collections::hash_map::RandomState;

    let mut set1: IndexSet<i32, RandomState> = IndexSet {
        map: IndexMap {
            core: IndexMapCore::new(),
            hash_builder: RandomState::new(),
        },
    };
    set1.insert(1);
    
    let mut set2: IndexSet<i32, RandomState> = IndexSet {
        map: IndexMap {
            core: IndexMapCore::new(),
            hash_builder: RandomState::new(),
        },
    };
    set2.insert(2);
    set2.insert(3);

    let result = set1.union(&set2);
    assert_eq!(result.iter().collect::<Vec<_>>(), vec![&1, &2, &3]);
}

