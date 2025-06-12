// Answer 0

#[test]
fn test_symmetric_difference_basic() {
    use hashbrown::HashSet;

    let a: HashSet<_> = vec![1, 2, 3].into_iter().collect();
    let b: HashSet<_> = vec![3, 4, 5].into_iter().collect();

    let set = &a ^ &b;

    let mut i = 0;
    let expected = [1, 2, 4, 5];
    for x in &set {
        assert!(expected.contains(x));
        i += 1;
    }
    assert_eq!(i, expected.len());
}

#[test]
fn test_symmetric_difference_empty() {
    use hashbrown::HashSet;

    let a: HashSet<_> = vec![].into_iter().collect();
    let b: HashSet<_> = vec![3, 4, 5].into_iter().collect();

    let set = &a ^ &b;

    let expected = [3, 4, 5];
    let mut i = 0;
    for x in &set {
        assert!(expected.contains(x));
        i += 1;
    }
    assert_eq!(i, expected.len());
}

#[test]
fn test_symmetric_difference_no_common_elements() {
    use hashbrown::HashSet;

    let a: HashSet<_> = vec![1, 2].into_iter().collect();
    let b: HashSet<_> = vec![3, 4].into_iter().collect();

    let set = &a ^ &b;

    let expected = [1, 2, 3, 4];
    let mut i = 0;
    for x in &set {
        assert!(expected.contains(x));
        i += 1;
    }
    assert_eq!(i, expected.len());
}

#[test]
fn test_symmetric_difference_all_common_elements() {
    use hashbrown::HashSet;

    let a: HashSet<_> = vec![1, 2, 3].into_iter().collect();
    let b: HashSet<_> = vec![1, 2, 3].into_iter().collect();

    let set = &a ^ &b;

    assert!(set.is_empty());
}

#[test]
fn test_symmetric_difference_with_self() {
    use hashbrown::HashSet;

    let a: HashSet<_> = vec![1, 2, 3].into_iter().collect();

    let set = &a ^ &a;

    assert!(set.is_empty());
}

