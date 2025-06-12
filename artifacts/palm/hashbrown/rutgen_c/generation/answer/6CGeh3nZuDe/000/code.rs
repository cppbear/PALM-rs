// Answer 0

#[test]
fn test_bitor_union_non_empty_sets() {
    use hashbrown::HashSet;

    let a: HashSet<_> = vec![1, 2, 3].into_iter().collect();
    let b: HashSet<_> = vec![3, 4, 5].into_iter().collect();

    let set = &a | &b;

    let expected = [1, 2, 3, 4, 5];
    let mut i = 0;
    for x in &set {
        assert!(expected.contains(x));
        i += 1;
    }
    assert_eq!(i, expected.len());
}

#[test]
fn test_bitor_union_empty_sets() {
    use hashbrown::HashSet;

    let a: HashSet<i32> = HashSet::new();
    let b: HashSet<i32> = HashSet::new();

    let set = &a | &b;

    assert!(set.is_empty());
}

#[test]
fn test_bitor_union_one_empty_one_non_empty() {
    use hashbrown::HashSet;

    let a: HashSet<_> = vec![1, 2, 3].into_iter().collect();
    let b: HashSet<i32> = HashSet::new();

    let set = &a | &b;

    let expected = [1, 2, 3];
    let mut i = 0;
    for x in &set {
        assert!(expected.contains(x));
        i += 1;
    }
    assert_eq!(i, expected.len());
}

#[test]
fn test_bitor_union_identical_sets() {
    use hashbrown::HashSet;

    let a: HashSet<_> = vec![1, 2, 3].into_iter().collect();
    let b: HashSet<_> = vec![1, 2, 3].into_iter().collect();

    let set = &a | &b;

    let expected = [1, 2, 3];
    let mut i = 0;
    for x in &set {
        assert!(expected.contains(x));
        i += 1;
    }
    assert_eq!(i, expected.len());
}

