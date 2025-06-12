// Answer 0

#[test]
fn test_sub_assign_different_sets() {
    use hashbrown::HashSet;

    let mut a: HashSet<i32> = vec![1, 2, 3, 6, 7].into_iter().collect();
    let b: HashSet<i32> = vec![3, 4, 5].into_iter().collect();

    a -= &b;

    let mut i = 0;
    let expected = [1, 2, 6, 7];
    for x in &a {
        assert!(expected.contains(x));
        i += 1;
    }
    assert_eq!(i, expected.len());
}

#[test]
fn test_sub_assign_with_empty_rhs() {
    use hashbrown::HashSet;

    let mut a: HashSet<i32> = vec![1, 2, 3].into_iter().collect();
    let b: HashSet<i32> = HashSet::new();

    a -= &b;

    let mut i = 0;
    let expected = [1, 2, 3];
    for x in &a {
        assert!(expected.contains(x));
        i += 1;
    }
    assert_eq!(i, expected.len());
}

#[test]
fn test_sub_assign_with_partial_overlap() {
    use hashbrown::HashSet;

    let mut a: HashSet<i32> = vec![1, 2, 3, 4].into_iter().collect();
    let b: HashSet<i32> = vec![3, 5].into_iter().collect();

    a -= &b;

    let mut i = 0;
    let expected = [1, 2, 4];
    for x in &a {
        assert!(expected.contains(x));
        i += 1;
    }
    assert_eq!(i, expected.len());
}

#[test]
fn test_sub_assign_same_length_rhs() {
    use hashbrown::HashSet;

    let mut a: HashSet<i32> = vec![1, 2, 3].into_iter().collect();
    let b: HashSet<i32> = vec![3, 4, 5].into_iter().collect();

    a -= &b;

    let mut i = 0;
    let expected = [1, 2];
    for x in &a {
        assert!(expected.contains(x));
        i += 1;
    }
    assert_eq!(i, expected.len());
}

