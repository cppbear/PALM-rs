// Answer 0

#[test]
fn test_hashset_difference_basic() {
    use hashbrown::HashSet;

    let a: HashSet<_> = vec![1, 2, 3].into_iter().collect();
    let b: HashSet<_> = vec![3, 4, 5].into_iter().collect();

    let set = &a - &b;

    let mut i = 0;
    let expected = [1, 2];
    for x in &set {
        assert!(expected.contains(x));
        i += 1;
    }
    assert_eq!(i, expected.len());
}

#[test]
fn test_hashset_difference_with_empty_set() {
    use hashbrown::HashSet;

    let a: HashSet<_> = vec![1, 2, 3].into_iter().collect();
    let b: HashSet<_> = HashSet::new();

    let set = &a - &b;

    assert_eq!(set.len(), 3);
    let expected = [1, 2, 3];
    for x in &set {
        assert!(expected.contains(x));
    }
}

#[test]
fn test_hashset_difference_with_identical_sets() {
    use hashbrown::HashSet;

    let a: HashSet<_> = vec![1, 2, 3].into_iter().collect();
    let b: HashSet<_> = vec![1, 2, 3].into_iter().collect();

    let set = &a - &b;

    assert_eq!(set.len(), 0);
}

#[test]
fn test_hashset_difference_with_non_overlapping_sets() {
    use hashbrown::HashSet;

    let a: HashSet<_> = vec![1, 2, 3].into_iter().collect();
    let b: HashSet<_> = vec![4, 5, 6].into_iter().collect();

    let set = &a - &b;

    assert_eq!(set.len(), 3);
    let expected = [1, 2, 3];
    for x in &set {
        assert!(expected.contains(x));
    }
}

