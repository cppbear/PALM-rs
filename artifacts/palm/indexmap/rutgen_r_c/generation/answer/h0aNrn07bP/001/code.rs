// Answer 0

#[test]
fn test_bitand_non_empty_sets() {
    use std::collections::hash_map::RandomState;
    
    let set1: super::IndexSet<i32, RandomState> = {
        let mut map = super::IndexMap::new();
        map.insert(1, ());
        map.insert(2, ());
        map.insert(3, ());
        super::IndexSet { map }
    };
    
    let set2: super::IndexSet<i32, RandomState> = {
        let mut map = super::IndexMap::new();
        map.insert(2, ());
        map.insert(3, ());
        map.insert(4, ());
        super::IndexSet { map }
    };

    let result = &set1 & &set2;

    let expected: Vec<_> = vec![2, 3];
    let result_vec: Vec<_> = result.map.keys().collect();
    assert_eq!(result_vec, expected);
}

#[test]
fn test_bitand_disjoint_sets() {
    use std::collections::hash_map::RandomState;
    
    let set1: super::IndexSet<i32, RandomState> = {
        let mut map = super::IndexMap::new();
        map.insert(1, ());
        map.insert(2, ());
        super::IndexSet { map }
    };

    let set2: super::IndexSet<i32, RandomState> = {
        let mut map = super::IndexMap::new();
        map.insert(3, ());
        map.insert(4, ());
        super::IndexSet { map }
    };

    let result = &set1 & &set2;

    let expected: Vec<i32> = Vec::new();
    let result_vec: Vec<_> = result.map.keys().collect();
    assert_eq!(result_vec, expected);
}

#[test]
fn test_bitand_empty_set() {
    use std::collections::hash_map::RandomState;
    
    let set1: super::IndexSet<i32, RandomState> = {
        let mut map = super::IndexMap::new();
        map.insert(1, ());
        map.insert(2, ());
        super::IndexSet { map }
    };

    let set2: super::IndexSet<i32, RandomState> = super::IndexSet {
        map: super::IndexMap::new(),
    };

    let result = &set1 & &set2;

    let expected: Vec<i32> = Vec::new();
    let result_vec: Vec<_> = result.map.keys().collect();
    assert_eq!(result_vec, expected);
}

#[test]
#[should_panic]
fn test_bitand_with_nonsense_input() {
    use std::collections::hash_map::RandomState;
    
    let set1: super::IndexSet<i32, RandomState> = {
        let mut map = super::IndexMap::new();
        map.insert(1, ());
        super::IndexSet { map }
    };

    let set2: super::IndexSet<String, RandomState> = {
        let mut map = super::IndexMap::new();
        map.insert("test".to_string(), ());
        super::IndexSet { map }
    };

    let _ = &set1 & &set2; // This should panic as types do not match
}

