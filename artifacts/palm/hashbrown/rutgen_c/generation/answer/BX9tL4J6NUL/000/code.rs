// Answer 0

#[test]
fn test_difference_basic() {
    use hashbrown::HashSet;

    let a: HashSet<_> = [1, 2, 3].into_iter().collect();
    let b: HashSet<_> = [4, 2, 3, 4].into_iter().collect();

    let diff: HashSet<_> = a.difference(&b).collect();
    assert_eq!(diff, [1].iter().collect::<HashSet<_>>());

    let diff_b: HashSet<_> = b.difference(&a).collect();
    assert_eq!(diff_b, [4].iter().collect::<HashSet<_>>());
}

#[test]
fn test_difference_empty() {
    use hashbrown::HashSet;

    let a: HashSet<_> = [].into_iter().collect();
    let b: HashSet<_> = [1, 2, 3].into_iter().collect();

    let diff: HashSet<_> = a.difference(&b).collect();
    assert_eq!(diff, [].iter().collect::<HashSet<_>>());

    let diff_b: HashSet<_> = b.difference(&a).collect();
    assert_eq!(diff_b, [1, 2, 3].iter().collect::<HashSet<_>>());
}

#[test]
fn test_difference_all_elements() {
    use hashbrown::HashSet;

    let a: HashSet<_> = [1, 2, 3].into_iter().collect();
    let b: HashSet<_> = [1, 2, 3].into_iter().collect();

    let diff: HashSet<_> = a.difference(&b).collect();
    assert_eq!(diff, [].iter().collect::<HashSet<_>>());
}

#[test]
fn test_difference_with_no_common_elements() {
    use hashbrown::HashSet;

    let a: HashSet<_> = [1, 2, 3].into_iter().collect();
    let b: HashSet<_> = [4, 5, 6].into_iter().collect();

    let diff: HashSet<_> = a.difference(&b).collect();
    assert_eq!(diff, [1, 2, 3].iter().collect::<HashSet<_>>());

    let diff_b: HashSet<_> = b.difference(&a).collect();
    assert_eq!(diff_b, [4, 5, 6].iter().collect::<HashSet<_>>());
}

