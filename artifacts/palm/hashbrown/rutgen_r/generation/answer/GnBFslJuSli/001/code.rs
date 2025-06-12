// Answer 0

#[test]
fn test_bitor_assign_with_no_common_elements() {
    use hashbrown::HashSet;

    let mut a: HashSet<i32> = vec![1, 2, 3].into_iter().collect();
    let b: HashSet<i32> = vec![4, 5, 6].into_iter().collect();

    a |= &b;

    let expected = [1, 2, 3, 4, 5, 6];
    let mut i = 0;
    for x in &a {
        assert!(expected.contains(x));
        i += 1;
    }
    assert_eq!(i, expected.len());
}

#[test]
fn test_bitor_assign_with_some_common_elements() {
    use hashbrown::HashSet;

    let mut a: HashSet<i32> = vec![1, 2, 3].into_iter().collect();
    let b: HashSet<i32> = vec![3, 4, 5].into_iter().collect();

    a |= &b;

    let expected = [1, 2, 3, 4, 5];
    let mut i = 0;
    for x in &a {
        assert!(expected.contains(x));
        i += 1;
    }
    assert_eq!(i, expected.len());
}

#[test]
fn test_bitor_assign_with_all_common_elements() {
    use hashbrown::HashSet;

    let mut a: HashSet<i32> = vec![1, 2, 3].into_iter().collect();
    let b: HashSet<i32> = vec![1, 2, 3].into_iter().collect();

    a |= &b;

    let expected = [1, 2, 3];
    let mut i = 0;
    for x in &a {
        assert!(expected.contains(x));
        i += 1;
    }
    assert_eq!(i, expected.len());
}

#[test]
fn test_bitor_assign_with_empty_rhs() {
    use hashbrown::HashSet;

    let mut a: HashSet<i32> = vec![1, 2, 3].into_iter().collect();
    let b: HashSet<i32> = HashSet::new();

    a |= &b;

    let expected = [1, 2, 3];
    let mut i = 0;
    for x in &a {
        assert!(expected.contains(x));
        i += 1;
    }
    assert_eq!(i, expected.len());
}

#[test]
fn test_bitor_assign_with_empty_self() {
    use hashbrown::HashSet;

    let mut a: HashSet<i32> = HashSet::new();
    let b: HashSet<i32> = vec![4, 5, 6].into_iter().collect();

    a |= &b;

    let expected = [4, 5, 6];
    let mut i = 0;
    for x in &a {
        assert!(expected.contains(x));
        i += 1;
    }
    assert_eq!(i, expected.len());
}

