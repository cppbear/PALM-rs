// Answer 0

#[test]
fn test_hashset_intersection_basic() {
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
fn test_hashset_intersection_no_common_elements() {
    use hashbrown::HashSet;

    let a: HashSet<_> = vec![1, 2, 3].into_iter().collect();
    let b: HashSet<_> = vec![4, 5, 6].into_iter().collect();

    let set = &a & &b;

    assert!(set.is_empty());
}

#[test]
fn test_hashset_intersection_equal_sets() {
    use hashbrown::HashSet;

    let a: HashSet<_> = vec![1, 2, 3].into_iter().collect();
    let b: HashSet<_> = vec![1, 2, 3].into_iter().collect();

    let set = &a & &b;

    let expected = [1, 2, 3];
    let mut i = 0;
    for x in &set {
        assert!(expected.contains(x));
        i += 1;
    }
    assert_eq!(i, expected.len());
}

#[test]
fn test_hashset_intersection_empty_first_set() {
    use hashbrown::HashSet;

    let a: HashSet<i32> = HashSet::new();
    let b: HashSet<_> = vec![2, 3, 4].into_iter().collect();

    let set = &a & &b;

    assert!(set.is_empty());
}

#[test]
fn test_hashset_intersection_empty_second_set() {
    use hashbrown::HashSet;

    let a: HashSet<_> = vec![1, 2, 3].into_iter().collect();
    let b: HashSet<i32> = HashSet::new();

    let set = &a & &b;

    assert!(set.is_empty());
}

#[test]
fn test_hashset_intersection_with_duplicates() {
    use hashbrown::HashSet;

    let a: HashSet<_> = vec![1, 2, 2, 3].into_iter().collect();
    let b: HashSet<_> = vec![2, 3, 4].into_iter().collect();

    let set = &a & &b;

    let expected = [2, 3];
    let mut i = 0;
    for x in &set {
        assert!(expected.contains(x));
        i += 1;
    }
    assert_eq!(i, expected.len());
}

