// Answer 0

#[test]
fn test_bitor_with_non_empty_sets() {
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
fn test_bitor_with_empty_set_a() {
    use hashbrown::HashSet;

    let a: HashSet<_> = HashSet::new();
    let b: HashSet<_> = vec![3, 4, 5].into_iter().collect();

    let set = &a | &b;

    let expected = [3, 4, 5];
    let mut i = 0;
    for x in &set {
        assert!(expected.contains(x));
        i += 1;
    }
    assert_eq!(i, expected.len());
}

#[test]
fn test_bitor_with_empty_set_b() {
    use hashbrown::HashSet;

    let a: HashSet<_> = vec![1, 2, 3].into_iter().collect();
    let b: HashSet<_> = HashSet::new();

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
fn test_bitor_with_identical_sets() {
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

#[test]
fn test_bitor_with_disjoint_sets() {
    use hashbrown::HashSet;

    let a: HashSet<_> = vec![1, 2].into_iter().collect();
    let b: HashSet<_> = vec![3, 4].into_iter().collect();

    let set = &a | &b;

    let expected = [1, 2, 3, 4];
    let mut i = 0;
    for x in &set {
        assert!(expected.contains(x));
        i += 1;
    }
    assert_eq!(i, expected.len());
}

