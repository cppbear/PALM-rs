// Answer 0

#[test]
fn test_hashset_difference_basic() {
    use hashbrown::HashSet;

    let a: HashSet<_> = vec![1, 2, 3].into_iter().collect();
    let b: HashSet<_> = vec![3, 4, 5].into_iter().collect();
    
    let set = &a - &b;

    let expected = [1, 2];
    let actual: Vec<_> = set.iter().cloned().collect();
    assert_eq!(actual, expected);
}

#[test]
fn test_hashset_difference_empty_rhs() {
    use hashbrown::HashSet;

    let a: HashSet<_> = vec![1, 2, 3].into_iter().collect();
    let b: HashSet<_> = HashSet::new();
    
    let set = &a - &b;

    let expected = [1, 2, 3];
    let actual: Vec<_> = set.iter().cloned().collect();
    assert_eq!(actual, expected);
}

#[test]
fn test_hashset_difference_empty_lhs() {
    use hashbrown::HashSet;

    let a: HashSet<_> = HashSet::new();
    let b: HashSet<_> = vec![3, 4, 5].into_iter().collect();
    
    let set = &a - &b;

    let expected: Vec<i32> = Vec::new();
    let actual: Vec<_> = set.iter().cloned().collect();
    assert_eq!(actual, expected);
}

#[test]
fn test_hashset_difference_no_common_elements() {
    use hashbrown::HashSet;

    let a: HashSet<_> = vec![1, 2].into_iter().collect();
    let b: HashSet<_> = vec![3, 4].into_iter().collect();
    
    let set = &a - &b;

    let expected = [1, 2];
    let actual: Vec<_> = set.iter().cloned().collect();
    assert_eq!(actual, expected);
}

#[test]
fn test_hashset_difference_all_common_elements() {
    use hashbrown::HashSet;

    let a: HashSet<_> = vec![1, 2, 3].into_iter().collect();
    let b: HashSet<_> = vec![1, 2, 3].into_iter().collect();
    
    let set = &a - &b;

    let expected: Vec<i32> = Vec::new();
    let actual: Vec<_> = set.iter().cloned().collect();
    assert_eq!(actual, expected);
}

#[test]
fn test_hashset_difference_with_duplicates_in_rhs() {
    use hashbrown::HashSet;

    let a: HashSet<_> = vec![1, 2, 3].into_iter().collect();
    let b: HashSet<_> = vec![2, 2, 3, 4].into_iter().collect(); // Duplicates are ignored in HashSet

    let set = &a - &b;

    let expected = [1];
    let actual: Vec<_> = set.iter().cloned().collect();
    assert_eq!(actual, expected);
}

