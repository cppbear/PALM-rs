// Answer 0

#[test]
fn test_difference_with_no_overlap() {
    let a: HashSet<_> = vec![1, 2, 3].into_iter().collect();
    let b: HashSet<_> = vec![4, 5, 6].into_iter().collect();

    let result = &a - &b;

    let expected: HashSet<_> = vec![1, 2, 3].into_iter().collect();
    assert_eq!(result, expected);
}

#[test]
fn test_difference_with_some_overlap() {
    let a: HashSet<_> = vec![1, 2, 3].into_iter().collect();
    let b: HashSet<_> = vec![2, 3, 4].into_iter().collect();

    let result = &a - &b;

    let expected: HashSet<_> = vec![1].into_iter().collect();
    assert_eq!(result, expected);
}

#[test]
fn test_difference_identical_sets() {
    let a: HashSet<_> = vec![1, 2, 3].into_iter().collect();
    let b: HashSet<_> = vec![1, 2, 3].into_iter().collect();

    let result = &a - &b;

    let expected: HashSet<_> = vec![].into_iter().collect();
    assert_eq!(result, expected);
}

#[test]
fn test_difference_empty_first_set() {
    let a: HashSet<_> = HashSet::new();
    let b: HashSet<_> = vec![1, 2, 3].into_iter().collect();

    let result = &a - &b;

    let expected: HashSet<_> = HashSet::new();
    assert_eq!(result, expected);
}

#[test]
fn test_difference_empty_second_set() {
    let a: HashSet<_> = vec![1, 2, 3].into_iter().collect();
    let b: HashSet<_> = HashSet::new();

    let result = &a - &b;

    let expected: HashSet<_> = vec![1, 2, 3].into_iter().collect();
    assert_eq!(result, expected);
}

