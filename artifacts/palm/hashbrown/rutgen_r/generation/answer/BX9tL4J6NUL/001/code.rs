// Answer 0

#[test]
fn test_difference_basic() {
    use hashbrown::HashSet;

    let a: HashSet<_> = [1, 2, 3].into_iter().collect();
    let b: HashSet<_> = [4, 2, 3, 4].into_iter().collect();

    let diff: HashSet<_> = a.difference(&b).collect();
    assert_eq!(diff, [1].iter().cloned().collect::<HashSet<_>>());
}

#[test]
fn test_difference_no_difference() {
    use hashbrown::HashSet;

    let a: HashSet<_> = [1, 2, 3].into_iter().collect();
    let b: HashSet<_> = [1, 2, 3].into_iter().collect();

    let diff: HashSet<_> = a.difference(&b).collect();
    assert!(diff.is_empty());
}

#[test]
fn test_difference_with_empty_other() {
    use hashbrown::HashSet;

    let a: HashSet<_> = [1, 2, 3].into_iter().collect();
    let b: HashSet<_> = [].into_iter().collect();

    let diff: HashSet<_> = a.difference(&b).collect();
    assert_eq!(diff, a);
}

#[test]
fn test_difference_other_empty() {
    use hashbrown::HashSet;

    let a: HashSet<_> = [].into_iter().collect();
    let b: HashSet<_> = [4, 5, 6].into_iter().collect();

    let diff: HashSet<_> = a.difference(&b).collect();
    assert!(diff.is_empty());
}

#[test]
fn test_difference_with_repeated_elements() {
    use hashbrown::HashSet;

    let a: HashSet<_> = [1, 2, 3, 1, 2, 3].into_iter().collect();
    let b: HashSet<_> = [3, 4, 2, 4].into_iter().collect();

    let diff: HashSet<_> = a.difference(&b).collect();
    assert_eq!(diff, [1].iter().cloned().collect::<HashSet<_>>());
}

#[test]
#[should_panic]
fn test_difference_panic_on_invalid_reference() {
    use hashbrown::HashSet;

    let a: HashSet<_> = [1, 2, 3].into_iter().collect();
    let b: HashSet<_> = [4, 5].into_iter().collect();
    
    let a_ref = &a;
    let _diff: HashSet<_> = a_ref.difference(&b).collect();
    drop(a); // This will cause a panic if a_ref is used after this line
}

