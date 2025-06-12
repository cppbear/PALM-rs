// Answer 0

#[test]
fn test_symmetric_difference() {
    use hashbrown::HashSet;

    let mut a: HashSet<_> = vec![1, 2, 3].into_iter().collect();
    let b: HashSet<_> = vec![3, 4, 5].into_iter().collect();

    a ^= &b;

    let mut i = 0;
    let expected = [1, 2, 4, 5];
    for x in &a {
        assert!(expected.contains(x));
        i += 1;
    }
    assert_eq!(i, expected.len());
}

#[test]
fn test_symmetric_difference_empty_sets() {
    use hashbrown::HashSet;

    let mut a: HashSet<i32> = HashSet::new();
    let b: HashSet<i32> = HashSet::new();

    a ^= &b;

    assert!(a.is_empty());
}

#[test]
fn test_symmetric_difference_one_empty_set() {
    use hashbrown::HashSet;

    let mut a: HashSet<_> = vec![1, 2, 3].into_iter().collect();
    let b: HashSet<_> = HashSet::new();

    a ^= &b;

    let expected = [1, 2, 3];
    for x in &a {
        assert!(expected.contains(x));
    }
    assert_eq!(a.len(), expected.len());
}

#[test]
fn test_symmetric_difference_same_elements() {
    use hashbrown::HashSet;

    let mut a: HashSet<_> = vec![1, 2, 3].into_iter().collect();
    let b: HashSet<_> = vec![1, 2, 3].into_iter().collect();

    a ^= &b;

    assert!(a.is_empty());
}

#[test]
fn test_symmetric_difference_complete_overlap() {
    use hashbrown::HashSet;

    let mut a: HashSet<_> = vec![1, 2, 3, 4].into_iter().collect();
    let b: HashSet<_> = vec![1, 2, 3, 4].into_iter().collect();

    a ^= &b;

    assert!(a.is_empty());
}

#[test]
fn test_symmetric_difference_no_overlap() {
    use hashbrown::HashSet;

    let mut a: HashSet<_> = vec![1, 2, 3].into_iter().collect();
    let b: HashSet<_> = vec![4, 5, 6].into_iter().collect();

    a ^= &b;

    let expected = [1, 2, 3, 4, 5, 6];
    let mut i = 0;
    for x in &a {
        assert!(expected.contains(x));
        i += 1;
    }
    assert_eq!(i, expected.len());
}

