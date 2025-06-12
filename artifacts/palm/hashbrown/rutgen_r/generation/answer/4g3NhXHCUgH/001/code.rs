// Answer 0

#[test]
fn test_union_equal_length_sets() {
    use hashbrown::HashSet;

    let a: HashSet<_> = [1, 2, 3].into_iter().collect();
    let b: HashSet<_> = [3, 2, 1].into_iter().collect();

    let union: HashSet<_> = a.union(&b).collect();
    assert_eq!(union.len(), 3);
    assert!(union.contains(&1));
    assert!(union.contains(&2));
    assert!(union.contains(&3));
}

#[test]
fn test_union_empty_set_with_non_empty_set() {
    use hashbrown::HashSet;

    let a: HashSet<i32> = HashSet::new();
    let b: HashSet<_> = [1, 2, 3].into_iter().collect();

    let union: HashSet<_> = a.union(&b).collect();
    assert_eq!(union.len(), 3);
    assert!(union.contains(&1));
    assert!(union.contains(&2));
    assert!(union.contains(&3));
}

#[test]
fn test_union_non_empty_set_with_empty_set() {
    use hashbrown::HashSet;

    let a: HashSet<_> = [1, 2, 3].into_iter().collect();
    let b: HashSet<i32> = HashSet::new();

    let union: HashSet<_> = a.union(&b).collect();
    assert_eq!(union.len(), 3);
    assert!(union.contains(&1));
    assert!(union.contains(&2));
    assert!(union.contains(&3));
}

#[test]
fn test_union_with_duplicates_in_other_set() {
    use hashbrown::HashSet;

    let a: HashSet<_> = [1, 2, 3].into_iter().collect();
    let b: HashSet<_> = [3, 3, 4, 4].into_iter().collect();

    let union: HashSet<_> = a.union(&b).collect();
    assert_eq!(union.len(), 4);
    assert!(union.contains(&1));
    assert!(union.contains(&2));
    assert!(union.contains(&3));
    assert!(union.contains(&4));
}

#[test]
fn test_union_identical_sets() {
    use hashbrown::HashSet;

    let a: HashSet<_> = [1, 2, 3].into_iter().collect();
    let b: HashSet<_> = [1, 2, 3].into_iter().collect();

    let union: HashSet<_> = a.union(&b).collect();
    assert_eq!(union.len(), 3);
    assert!(union.contains(&1));
    assert!(union.contains(&2));
    assert!(union.contains(&3));
}

