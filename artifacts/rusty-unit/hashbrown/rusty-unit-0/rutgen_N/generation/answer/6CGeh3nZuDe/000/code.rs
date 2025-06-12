// Answer 0

#[test]
fn test_hashset_union() {
    use hashbrown::HashSet;

    let a: HashSet<_> = vec![1, 2, 3].into_iter().collect();
    let b: HashSet<_> = vec![3, 4, 5].into_iter().collect();

    let set = &a | &b;

    let mut i = 0;
    let expected = [1, 2, 3, 4, 5];
    for x in &set {
        assert!(expected.contains(x));
        i += 1;
    }
    assert_eq!(i, expected.len());
}

#[test]
fn test_hashset_union_empty() {
    use hashbrown::HashSet;

    let a: HashSet<i32> = HashSet::new();
    let b: HashSet<i32> = HashSet::new();

    let set = &a | &b;

    assert!(set.is_empty());
}

#[test]
fn test_hashset_union_one_empty() {
    use hashbrown::HashSet;

    let a: HashSet<_> = vec![1, 2, 3].into_iter().collect();
    let b: HashSet<i32> = HashSet::new();

    let set = &a | &b;

    let mut i = 0;
    let expected = [1, 2, 3];
    for x in &set {
        assert!(expected.contains(x));
        i += 1;
    }
    assert_eq!(i, expected.len());
}

