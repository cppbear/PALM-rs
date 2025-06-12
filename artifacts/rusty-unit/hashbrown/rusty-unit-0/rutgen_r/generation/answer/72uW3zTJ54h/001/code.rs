// Answer 0

#[test]
fn test_intersection_non_empty_sets() {
    use hashbrown::HashSet;

    let a: HashSet<_> = vec![1, 2, 3].into_iter().collect();
    let b: HashSet<_> = vec![2, 3, 4].into_iter().collect();
    let set = &a & &b;

    let mut i = 0;
    let expected = [2, 3];
    for x in &set {
        assert!(expected.contains(x));
        i += 1;
    }
    assert_eq!(i, expected.len());
}

#[test]
fn test_intersection_empty_set() {
    use hashbrown::HashSet;

    let a: HashSet<_> = vec![1, 2, 3].into_iter().collect();
    let b: HashSet<_> = HashSet::new();
    let set = &a & &b;

    assert!(set.is_empty());
}

#[test]
fn test_intersection_no_common_elements() {
    use hashbrown::HashSet;

    let a: HashSet<_> = vec![1, 2, 3].into_iter().collect();
    let b: HashSet<_> = vec![4, 5, 6].into_iter().collect();
    let set = &a & &b;

    assert!(set.is_empty());
}

#[test]
fn test_intersection_identical_sets() {
    use hashbrown::HashSet;

    let a: HashSet<_> = vec![1, 2, 3].into_iter().collect();
    let b: HashSet<_> = a.clone();
    let set = &a & &b;

    let mut i = 0;
    for x in &set {
        assert!(a.contains(x));
        i += 1;
    }
    assert_eq!(i, a.len());
}

#[test]
fn test_intersection_with_duplicates_in_a() {
    use hashbrown::HashSet;

    let a: HashSet<_> = vec![1, 1, 2, 3].into_iter().collect();
    let b: HashSet<_> = vec![2, 3, 4].into_iter().collect();
    let set = &a & &b;

    let mut i = 0;
    let expected = [2, 3];
    for x in &set {
        assert!(expected.contains(x));
        i += 1;
    }
    assert_eq!(i, expected.len());
}

