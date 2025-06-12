// Answer 0

#[test]
fn test_intersection_basic() {
    let a: HashSet<_> = vec![1, 2, 3].into_iter().collect();
    let b: HashSet<_> = vec![2, 3, 4].into_iter().collect();

    let set = &a & &b;

    let expected: Vec<_> = vec![2, 3];
    let mut results: Vec<_> = set.iter().cloned().collect();
    results.sort();
    expected.sort();

    assert_eq!(results, expected);
}

#[test]
fn test_intersection_empty_sets() {
    let a: HashSet<i32> = HashSet::new();
    let b: HashSet<i32> = HashSet::new();

    let set = &a & &b;

    assert!(set.is_empty());
}

#[test]
fn test_intersection_one_empty_set() {
    let a: HashSet<_> = vec![1, 2, 3].into_iter().collect();
    let b: HashSet<i32> = HashSet::new();

    let set = &a & &b;

    assert!(set.is_empty());
}

#[test]
fn test_intersection_no_common_elements() {
    let a: HashSet<_> = vec![1, 2].into_iter().collect();
    let b: HashSet<_> = vec![3, 4].into_iter().collect();

    let set = &a & &b;

    assert!(set.is_empty());
}

#[test]
fn test_intersection_same_elements() {
    let a: HashSet<_> = vec![1, 2, 3].into_iter().collect();
    let b: HashSet<_> = vec![1, 2, 3].into_iter().collect();

    let set = &a & &b;

    let expected: Vec<_> = vec![1, 2, 3];
    let mut results: Vec<_> = set.iter().cloned().collect();
    results.sort();
    expected.sort();

    assert_eq!(results, expected);
}

#[test]
fn test_intersection_with_duplicates() {
    let a: HashSet<_> = vec![1, 2, 2, 3].into_iter().collect();
    let b: HashSet<_> = vec![2, 2, 3, 4].into_iter().collect();

    let set = &a & &b;

    let expected: Vec<_> = vec![2, 3];
    let mut results: Vec<_> = set.iter().cloned().collect();
    results.sort();
    expected.sort();

    assert_eq!(results, expected);
}

