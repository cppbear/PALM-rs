// Answer 0

#[test]
fn test_symmetric_difference_unique_elements() {
    use hashbrown::HashSet;

    let a: HashSet<_> = [1, 2, 3].iter().cloned().collect();
    let b: HashSet<_> = [4, 2, 3, 4].iter().cloned().collect();
    
    let diff: HashSet<_> = a.symmetric_difference(&b).collect();
    
    assert_eq!(diff, [1, 4].iter().cloned().collect::<HashSet<_>>());
}

#[test]
fn test_symmetric_difference_no_elements() {
    use hashbrown::HashSet;

    let a: HashSet<i32> = HashSet::new();
    let b: HashSet<i32> = HashSet::new();
    
    let diff: HashSet<_> = a.symmetric_difference(&b).collect();
    
    assert!(diff.is_empty());
}

#[test]
fn test_symmetric_difference_identical_sets() {
    use hashbrown::HashSet;

    let a: HashSet<_> = [1, 2, 3].iter().cloned().collect();
    let b: HashSet<_> = [1, 2, 3].iter().cloned().collect();
    
    let diff: HashSet<_> = a.symmetric_difference(&b).collect();
    
    assert!(diff.is_empty());
}

#[test]
fn test_symmetric_difference_one_empty_set() {
    use hashbrown::HashSet;

    let a: HashSet<_> = [1, 2, 3].iter().cloned().collect();
    let b: HashSet<i32> = HashSet::new();
    
    let diff: HashSet<_> = a.symmetric_difference(&b).collect();
    
    assert_eq!(diff, a);
    
    let diff_other: HashSet<_> = b.symmetric_difference(&a).collect();
    
    assert_eq!(diff_other, b);
}

#[test]
fn test_symmetric_difference_multiple_common_elements() {
    use hashbrown::HashSet;

    let a: HashSet<_> = [1, 2, 3, 5].iter().cloned().collect();
    let b: HashSet<_> = [2, 3, 4, 5, 6].iter().cloned().collect();
    
    let diff: HashSet<_> = a.symmetric_difference(&b).collect();
    
    assert_eq!(diff, [1, 4, 6].iter().cloned().collect::<HashSet<_>>());
}

