// Answer 0

#[test]
fn test_hashset_difference_some_elements() {
    use hashbrown::HashSet;

    let a: HashSet<_> = vec![1, 2, 3].into_iter().collect();
    let b: HashSet<_> = vec![3, 4, 5].into_iter().collect();
    
    let set = &a - &b;

    let mut i = 0;
    let expected = [1, 2];
    for x in &set {
        assert!(expected.contains(x));
        i += 1;
    }
    assert_eq!(i, expected.len());
}

#[test]
fn test_hashset_difference_no_elements() {
    use hashbrown::HashSet;

    let a: HashSet<_> = vec![1, 2, 3].into_iter().collect();
    let b: HashSet<_> = vec![1, 2, 3].into_iter().collect();
    
    let set = &a - &b;

    assert!(set.is_empty());
}

#[test]
fn test_hashset_difference_all_elements() {
    use hashbrown::HashSet;

    let a: HashSet<_> = vec![1, 2, 3].into_iter().collect();
    let b: HashSet<_> = vec![4, 5, 6].into_iter().collect();
    
    let set = &a - &b;

    assert_eq!(set.len(), a.len());
    for x in &set {
        assert!(a.contains(x));
    }
}

#[test]
fn test_hashset_difference_empty_set() {
    use hashbrown::HashSet;

    let a: HashSet<_> = vec![1, 2, 3].into_iter().collect();
    let b: HashSet<_> = HashSet::new();
    
    let set = &a - &b;

    assert_eq!(set.len(), a.len());
    for x in &set {
        assert!(a.contains(x));
    }
}

