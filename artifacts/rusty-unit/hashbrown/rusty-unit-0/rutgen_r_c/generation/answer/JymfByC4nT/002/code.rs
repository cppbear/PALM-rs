// Answer 0

#[test]
fn test_sub_assign_rhs_smaller() {
    use hashbrown::HashSet;

    let mut a: HashSet<_> = vec![1, 2, 3, 4, 5].into_iter().collect();
    let b: HashSet<_> = vec![3].into_iter().collect();

    a -= &b;

    let mut i = 0;
    let expected = [1, 2, 4, 5];
    for x in &a {
        assert!(expected.contains(x));
        i += 1;
    }
    assert_eq!(i, expected.len());
}

#[test]
fn test_sub_assign_no_change() {
    use hashbrown::HashSet;

    let mut a: HashSet<_> = vec![1, 2, 3].into_iter().collect();
    let b: HashSet<_> = vec![2, 3].into_iter().collect();

    a -= &b;

    let mut i = 0;
    let expected = [1];
    for x in &a {
        assert!(expected.contains(x));
        i += 1;
    }
    assert_eq!(i, expected.len());
}

#[test]
fn test_sub_assign_empty_rhs() {
    use hashbrown::HashSet;

    let mut a: HashSet<_> = vec![1, 2, 3].into_iter().collect();
    let b: HashSet<_> = vec![].into_iter().collect();

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
fn test_sub_assign_no_elements_left() {
    use hashbrown::HashSet;

    let mut a: HashSet<_> = vec![1, 2, 3].into_iter().collect();
    let b: HashSet<_> = vec![1, 2, 3].into_iter().collect();

    a -= &b;

    assert!(a.is_empty());
}

