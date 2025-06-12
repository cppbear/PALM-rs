// Answer 0

#[test]
fn test_intersection_non_empty_sets() {
    use hashbrown::HashSet;

    let mut a: HashSet<_> = vec![1, 2, 3].into_iter().collect();
    let b: HashSet<_> = vec![2, 3, 4].into_iter().collect();
    
    a &= &b;

    let expected: HashSet<_> = vec![2, 3].into_iter().collect();
    assert_eq!(a, expected);
}

#[test]
fn test_intersection_empty_first_set() {
    use hashbrown::HashSet;

    let mut a: HashSet<_> = HashSet::new();
    let b: HashSet<_> = vec![2, 3, 4].into_iter().collect();
    
    a &= &b;

    let expected: HashSet<_> = HashSet::new();
    assert_eq!(a, expected);
}

#[test]
fn test_intersection_empty_second_set() {
    use hashbrown::HashSet;

    let mut a: HashSet<_> = vec![1, 2, 3].into_iter().collect();
    let b: HashSet<_> = HashSet::new();
    
    a &= &b;

    let expected: HashSet<_> = HashSet::new();
    assert_eq!(a, expected);
}

#[test]
fn test_intersection_no_common_elements() {
    use hashbrown::HashSet;

    let mut a: HashSet<_> = vec![1, 5, 6].into_iter().collect();
    let b: HashSet<_> = vec![2, 3, 4].into_iter().collect();
    
    a &= &b;

    let expected: HashSet<_> = HashSet::new();
    assert_eq!(a, expected);
}

#[test]
fn test_intersection_same_elements() {
    use hashbrown::HashSet;

    let mut a: HashSet<_> = vec![1, 2, 3].into_iter().collect();
    let b: HashSet<_> = vec![1, 2, 3].into_iter().collect();
    
    a &= &b;

    let expected: HashSet<_> = vec![1, 2, 3].into_iter().collect();
    assert_eq!(a, expected);
}

