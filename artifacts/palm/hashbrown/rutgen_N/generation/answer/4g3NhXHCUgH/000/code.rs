// Answer 0

#[test]
fn test_union_no_duplicates() {
    use hashbrown::HashSet;

    let a: HashSet<_> = [1, 2, 3].into_iter().collect();
    let b: HashSet<_> = [4, 2, 3, 4].into_iter().collect();

    let union: HashSet<_> = a.union(&b).collect();
    assert_eq!(union, [1, 2, 3, 4].iter().collect::<HashSet<_>>());
}

#[test]
fn test_union_with_empty_set() {
    use hashbrown::HashSet;

    let a: HashSet<_> = [].into_iter().collect();
    let b: HashSet<_> = [1, 2, 3].into_iter().collect();

    let union: HashSet<_> = a.union(&b).collect();
    assert_eq!(union, [1, 2, 3].iter().collect::<HashSet<_>>());
}

#[test]
fn test_union_identical_sets() {
    use hashbrown::HashSet;

    let a: HashSet<_> = [1, 2, 3].into_iter().collect();
    let b: HashSet<_> = [1, 2, 3].into_iter().collect();

    let union: HashSet<_> = a.union(&b).collect();
    assert_eq!(union, [1, 2, 3].iter().collect::<HashSet<_>>());
}

#[test]
fn test_union_large_differing_sets() {
    use hashbrown::HashSet;

    let a: HashSet<_> = [1, 2, 3, 5].into_iter().collect();
    let b: HashSet<_> = [4, 5, 6, 7].into_iter().collect();

    let union: HashSet<_> = a.union(&b).collect();
    assert_eq!(union, [1, 2, 3, 4, 5, 6, 7].iter().collect::<HashSet<_>>());
}

