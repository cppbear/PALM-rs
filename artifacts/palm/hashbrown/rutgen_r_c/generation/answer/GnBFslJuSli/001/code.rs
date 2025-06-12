// Answer 0

#[test]
fn test_bitor_assign_with_no_common_elements() {
    let mut a: HashSet<i32> = vec![1, 2].into_iter().collect();
    let b: HashSet<i32> = vec![3, 4].into_iter().collect();

    a |= &b;

    let expected: Vec<i32> = vec![1, 2, 3, 4];
    for x in &a {
        assert!(expected.contains(x));
    }
    assert_eq!(a.len(), expected.len());
}

#[test]
fn test_bitor_assign_with_some_common_elements() {
    let mut a: HashSet<i32> = vec![1, 2, 3].into_iter().collect();
    let b: HashSet<i32> = vec![3, 4, 5].into_iter().collect();

    a |= &b;

    let expected: Vec<i32> = vec![1, 2, 3, 4, 5];
    for x in &a {
        assert!(expected.contains(x));
    }
    assert_eq!(a.len(), expected.len());
}

#[test]
fn test_bitor_assign_with_all_common_elements() {
    let mut a: HashSet<i32> = vec![1, 2, 3].into_iter().collect();
    let b: HashSet<i32> = vec![1, 2, 3].into_iter().collect();

    a |= &b;

    let expected: Vec<i32> = vec![1, 2, 3];
    for x in &a {
        assert!(expected.contains(x));
    }
    assert_eq!(a.len(), expected.len());
}

#[test]
fn test_bitor_assign_with_empty_rhs() {
    let mut a: HashSet<i32> = vec![1, 2, 3].into_iter().collect();
    let b: HashSet<i32> = HashSet::new(); // Empty HashSet

    a |= &b;

    let expected: Vec<i32> = vec![1, 2, 3];
    for x in &a {
        assert!(expected.contains(x));
    }
    assert_eq!(a.len(), expected.len());
}

#[test]
fn test_bitor_assign_with_empty_self() {
    let mut a: HashSet<i32> = HashSet::new(); // Empty HashSet
    let b: HashSet<i32> = vec![3, 4, 5].into_iter().collect();

    a |= &b;

    let expected: Vec<i32> = vec![3, 4, 5];
    for x in &a {
        assert!(expected.contains(x));
    }
    assert_eq!(a.len(), expected.len());
}

