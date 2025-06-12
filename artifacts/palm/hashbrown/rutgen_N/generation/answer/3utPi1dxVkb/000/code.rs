// Answer 0

#[test]
fn test_intersection_with_non_empty_sets() {
    use hashbrown::HashSet;

    let mut a: HashSet<_> = vec![1, 2, 3].into_iter().collect();
    let b: HashSet<_> = vec![2, 3, 4].into_iter().collect();
    
    a &= &b;

    let expected = [2, 3];
    let mut i = 0;
    for x in &a {
        assert!(expected.contains(x));
        i += 1;
    }
    assert_eq!(i, expected.len());
}

#[test]
fn test_intersection_with_empty_set() {
    use hashbrown::HashSet;

    let mut a: HashSet<_> = vec![1, 2, 3].into_iter().collect();
    let b: HashSet<_> = HashSet::new();
    
    a &= &b;

    assert!(a.is_empty());
}

#[test]
fn test_intersection_with_identical_sets() {
    use hashbrown::HashSet;

    let mut a: HashSet<_> = vec![1, 2, 3].into_iter().collect();
    let b: HashSet<_> = vec![1, 2, 3].into_iter().collect();
    
    a &= &b;

    assert_eq!(a.len(), 3);
    assert!(a.contains(&1));
    assert!(a.contains(&2));
    assert!(a.contains(&3));
}

#[test]
fn test_intersection_with_no_common_elements() {
    use hashbrown::HashSet;

    let mut a: HashSet<_> = vec![1, 2, 3].into_iter().collect();
    let b: HashSet<_> = vec![4, 5, 6].into_iter().collect();
    
    a &= &b;

    assert!(a.is_empty());
}

