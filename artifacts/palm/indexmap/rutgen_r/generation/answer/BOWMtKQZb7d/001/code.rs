// Answer 0

#[test]
fn test_union_empty_sets() {
    use std::collections::hash_map::RandomState;
    use indexmap::IndexSet;

    let set1: IndexSet<i32, RandomState> = IndexSet::new();
    let set2: IndexSet<i32, RandomState> = IndexSet::new();
    
    let result: Vec<_> = set1.union(&set2).collect();
    assert_eq!(result, Vec::<i32>::new());
}

#[test]
fn test_union_with_common_elements() {
    use std::collections::hash_map::RandomState;
    use indexmap::IndexSet;

    let mut set1: IndexSet<i32, RandomState> = IndexSet::new();
    set1.insert(1);
    set1.insert(2);

    let mut set2: IndexSet<i32, RandomState> = IndexSet::new();
    set2.insert(2);
    set2.insert(3);
    
    let result: Vec<_> = set1.union(&set2).collect();
    assert_eq!(result, vec![1, 2, 3]);
}

#[test]
fn test_union_with_unique_elements() {
    use std::collections::hash_map::RandomState;
    use indexmap::IndexSet;

    let mut set1: IndexSet<i32, RandomState> = IndexSet::new();
    set1.insert(1);
    set1.insert(2);

    let mut set2: IndexSet<i32, RandomState> = IndexSet::new();
    set2.insert(3);
    set2.insert(4);
    
    let result: Vec<_> = set1.union(&set2).collect();
    assert_eq!(result, vec![1, 2, 3, 4]);
}

#[test]
fn test_union_with_repeated_calls() {
    use std::collections::hash_map::RandomState;
    use indexmap::IndexSet;

    let mut set1: IndexSet<i32, RandomState> = IndexSet::new();
    set1.insert(1);
    
    let mut set2: IndexSet<i32, RandomState> = IndexSet::new();
    set2.insert(1);
    set2.insert(2);
    
    let result1: Vec<_> = set1.union(&set2).collect();
    assert_eq!(result1, vec![1, 2]);

    let result2: Vec<_> = set2.union(&set1).collect();
    assert_eq!(result2, vec![1, 2]);
}

#[test]
fn test_union_large_sets() {
    use std::collections::hash_map::RandomState;
    use indexmap::IndexSet;

    let mut set1: IndexSet<i32, RandomState> = IndexSet::new();
    for i in 0..1000 {
        set1.insert(i);
    }

    let mut set2: IndexSet<i32, RandomState> = IndexSet::new();
    for i in 500..1500 {
        set2.insert(i);
    }
    
    let result: Vec<_> = set1.union(&set2).collect();
    assert_eq!(result.len(), 1500);
    for i in 0..1500 {
        assert!(result.contains(&i));
    }
}

