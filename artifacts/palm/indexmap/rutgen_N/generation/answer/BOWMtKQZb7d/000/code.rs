// Answer 0

#[test]
fn test_union_with_overlapping_values() {
    use std::collections::hash_map::RandomState;
    use crate::IndexSet;

    let mut set1 = IndexSet::new();
    set1.insert(1);
    set1.insert(2);
    set1.insert(3);

    let mut set2 = IndexSet::new();
    set2.insert(3);
    set2.insert(4);
    set2.insert(5);

    let union_set = set1.union(&set2).collect::<Vec<_>>();

    assert_eq!(union_set, vec![1, 2, 3, 4, 5]);
}

#[test]
fn test_union_with_non_overlapping_values() {
    use std::collections::hash_map::RandomState;
    use crate::IndexSet;

    let mut set1 = IndexSet::new();
    set1.insert(1);
    set1.insert(2);
    
    let mut set2 = IndexSet::new();
    set2.insert(3);
    set2.insert(4);

    let union_set = set1.union(&set2).collect::<Vec<_>>();

    assert_eq!(union_set, vec![1, 2, 3, 4]);
}

#[test]
fn test_union_with_empty_set() {
    use std::collections::hash_map::RandomState;
    use crate::IndexSet;

    let mut set1 = IndexSet::new();
    set1.insert(1);
    set1.insert(2);

    let set2 = IndexSet::new();

    let union_set = set1.union(&set2).collect::<Vec<_>>();

    assert_eq!(union_set, vec![1, 2]);
}

#[test]
fn test_union_of_two_empty_sets() {
    use std::collections::hash_map::RandomState;
    use crate::IndexSet;

    let set1 = IndexSet::new();
    let set2 = IndexSet::new();

    let union_set = set1.union(&set2).collect::<Vec<_>>();

    assert_eq!(union_set, Vec::<i32>::new());
}

