// Answer 0

#[test]
fn test_bitor_union_with_unique_elements() {
    use std::collections::hash_map::RandomState;
    use core::hash::Hash;

    #[derive(Eq, PartialEq, Hash, Clone)]
    struct TestType(u32);

    let set1: super::IndexSet<TestType, RandomState> = {
        let mut map = super::IndexMap::new();
        map.insert(TestType(1), ());
        map.insert(TestType(2), ());
        super::IndexSet { map }
    };

    let set2: super::IndexSet<TestType, RandomState> = {
        let mut map = super::IndexMap::new();
        map.insert(TestType(2), ());
        map.insert(TestType(3), ());
        super::IndexSet { map }
    };

    let result = set1.bitor(&set2);
    let expected: super::IndexSet<TestType, RandomState> = {
        let mut map = super::IndexMap::new();
        map.insert(TestType(1), ());
        map.insert(TestType(2), ());
        map.insert(TestType(3), ());
        super::IndexSet { map }
    };

    assert_eq!(result, expected);
}

#[test]
fn test_bitor_union_with_no_unique_elements() {
    use std::collections::hash_map::RandomState;
    use core::hash::Hash;

    #[derive(Eq, PartialEq, Hash, Clone)]
    struct TestType(u32);

    let set1: super::IndexSet<TestType, RandomState> = {
        let mut map = super::IndexMap::new();
        map.insert(TestType(1), ());
        map.insert(TestType(2), ());
        super::IndexSet { map }
    };

    let set2: super::IndexSet<TestType, RandomState> = {
        let mut map = super::IndexMap::new();
        map.insert(TestType(1), ());
        map.insert(TestType(2), ());
        super::IndexSet { map }
    };

    let result = set1.bitor(&set2);
    let expected: super::IndexSet<TestType, RandomState> = {
        let mut map = super::IndexMap::new();
        map.insert(TestType(1), ());
        map.insert(TestType(2), ());
        super::IndexSet { map }
    };

    assert_eq!(result, expected);
}

#[test]
fn test_bitor_empty_sets() {
    use std::collections::hash_map::RandomState;
    use core::hash::Hash;

    #[derive(Eq, PartialEq, Hash, Clone)]
    struct TestType(u32);

    let set1: super::IndexSet<TestType, RandomState> = {
        let mut map = super::IndexMap::new();
        super::IndexSet { map }
    };

    let set2: super::IndexSet<TestType, RandomState> = {
        let mut map = super::IndexMap::new();
        super::IndexSet { map }
    };

    let result = set1.bitor(&set2);
    let expected: super::IndexSet<TestType, RandomState> = {
        let mut map = super::IndexMap::new();
        super::IndexSet { map }
    };

    assert_eq!(result, expected);
}

