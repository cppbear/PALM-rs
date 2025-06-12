// Answer 0

#[test]
fn test_symmetric_difference() {
    use crate::HashSet;
    use std::collections::hash_map::RandomState;

    let a: HashSet<i32, RandomState> = vec![1, 2, 3].into_iter().collect();
    let b: HashSet<i32, RandomState> = vec![3, 4, 5].into_iter().collect();

    let set = &a ^ &b;

    let mut i = 0;
    let expected = [1, 2, 4, 5];
    for x in &set {
        assert!(expected.contains(x));
        i += 1;
    }
    assert_eq!(i, expected.len());
}

#[test]
fn test_symmetric_difference_empty_sets() {
    use crate::HashSet;
    use std::collections::hash_map::RandomState;

    let a: HashSet<i32, RandomState> = HashSet::new();
    let b: HashSet<i32, RandomState> = HashSet::new();

    let set = &a ^ &b;

    assert!(set.is_empty());
}

#[test]
fn test_symmetric_difference_disjoint_sets() {
    use crate::HashSet;
    use std::collections::hash_map::RandomState;

    let a: HashSet<i32, RandomState> = vec![1, 2].into_iter().collect();
    let b: HashSet<i32, RandomState> = vec![3, 4].into_iter().collect();

    let set = &a ^ &b;

    let mut i = 0;
    let expected = [1, 2, 3, 4];
    for x in &set {
        assert!(expected.contains(x));
        i += 1;
    }
    assert_eq!(i, expected.len());
}

#[test]
fn test_symmetric_difference_with_common_element() {
    use crate::HashSet;
    use std::collections::hash_map::RandomState;

    let a: HashSet<i32, RandomState> = vec![1, 2, 3].into_iter().collect();
    let b: HashSet<i32, RandomState> = vec![2, 3, 4].into_iter().collect();

    let set = &a ^ &b;

    let mut i = 0;
    let expected = [1, 4];
    for x in &set {
        assert!(expected.contains(x));
        i += 1;
    }
    assert_eq!(i, expected.len());
}

