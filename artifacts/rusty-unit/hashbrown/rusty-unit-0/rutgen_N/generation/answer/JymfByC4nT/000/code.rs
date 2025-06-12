// Answer 0

#[test]
fn test_sub_assign_with_non_overlapping_sets() {
    use hashbrown::HashSet;

    let mut a: HashSet<_> = vec![1, 2, 3].into_iter().collect();
    let b: HashSet<_> = vec![4, 5, 6].into_iter().collect();

    a -= &b;

    let expected = [1, 2, 3];
    let actual: Vec<_> = a.iter().cloned().collect();
    assert_eq!(actual, expected);
}

#[test]
fn test_sub_assign_with_some_overlapping_elements() {
    use hashbrown::HashSet;

    let mut a: HashSet<_> = vec![1, 2, 3].into_iter().collect();
    let b: HashSet<_> = vec![2, 3, 4].into_iter().collect();

    a -= &b;

    let expected = [1];
    let actual: Vec<_> = a.iter().cloned().collect();
    assert_eq!(actual, expected);
}

#[test]
fn test_sub_assign_with_completely_overlapping_sets() {
    use hashbrown::HashSet;

    let mut a: HashSet<_> = vec![1, 2, 3].into_iter().collect();
    let b: HashSet<_> = vec![1, 2, 3].into_iter().collect();

    a -= &b;

    let expected: Vec<i32> = vec![];
    let actual: Vec<_> = a.iter().cloned().collect();
    assert_eq!(actual, expected);
}

#[test]
fn test_sub_assign_with_empty_rhs() {
    use hashbrown::HashSet;

    let mut a: HashSet<_> = vec![1, 2, 3].into_iter().collect();
    let b: HashSet<_> = HashSet::new();

    a -= &b;

    let expected = [1, 2, 3];
    let actual: Vec<_> = a.iter().cloned().collect();
    assert_eq!(actual, expected);
}

#[test]
fn test_sub_assign_with_large_rhs() {
    use hashbrown::HashSet;

    let mut a: HashSet<_> = vec![1, 2, 3].into_iter().collect();
    let b: HashSet<_> = (1..=100).collect();

    a -= &b;

    let expected = [2, 3]; // Only 2 and 3 remain as 1 is removed
    let actual: Vec<_> = a.iter().cloned().collect();
    assert_eq!(actual, expected);
}

