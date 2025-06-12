// Answer 0

#[test]
fn test_symmetric_difference_no_common_elements() {
    use hashbrown::HashSet;

    let a: HashSet<_> = [1, 2, 3].into_iter().collect();
    let b: HashSet<_> = [4, 5, 6].into_iter().collect();

    let diff: HashSet<_> = a.symmetric_difference(&b).collect();

    assert_eq!(diff, [1, 2, 3, 4, 5, 6].iter().collect::<HashSet<_>>());
}

#[test]
fn test_symmetric_difference_some_common_elements() {
    use hashbrown::HashSet;

    let a: HashSet<_> = [1, 2, 3].into_iter().collect();
    let b: HashSet<_> = [3, 4, 5].into_iter().collect();

    let diff: HashSet<_> = a.symmetric_difference(&b).collect();

    assert_eq!(diff, [1, 2, 4, 5].iter().collect::<HashSet<_>>());
}

#[test]
fn test_symmetric_difference_all_common_elements() {
    use hashbrown::HashSet;

    let a: HashSet<_> = [1, 2, 3].into_iter().collect();
    let b: HashSet<_> = [1, 2, 3].into_iter().collect();

    let diff: HashSet<_> = a.symmetric_difference(&b).collect();

    assert_eq!(diff.len(), 0);
}

#[test]
fn test_symmetric_difference_empty_sets() {
    use hashbrown::HashSet;

    let a: HashSet<i32> = HashSet::new();
    let b: HashSet<i32> = HashSet::new();

    let diff: HashSet<_> = a.symmetric_difference(&b).collect();

    assert_eq!(diff.len(), 0);
}

#[test]
fn test_symmetric_difference_one_empty_set() {
    use hashbrown::HashSet;

    let a: HashSet<_> = [1, 2, 3].into_iter().collect();
    let b: HashSet<i32> = HashSet::new();

    let diff: HashSet<_> = a.symmetric_difference(&b).collect();

    assert_eq!(diff, [1, 2, 3].iter().collect::<HashSet<_>>());
}

