// Answer 0

#[test]
fn test_bitxor_assign_with_non_overlapping_items() {
    let mut a: HashSet<i32> = vec![1, 2, 3].into_iter().collect();
    let b: HashSet<i32> = vec![4, 5].into_iter().collect();

    a ^= &b;

    let expected = [1, 2, 3, 4, 5];
    let result: Vec<_> = a.iter().collect();
    assert_eq!(result.len(), expected.len());
    for &x in &expected {
        assert!(result.contains(&&x));
    }
}

#[test]
fn test_bitxor_assign_with_common_items() {
    let mut a: HashSet<i32> = vec![1, 2, 3].into_iter().collect();
    let b: HashSet<i32> = vec![3, 4, 5].into_iter().collect();

    a ^= &b;

    let expected = [1, 2, 4, 5];
    let result: Vec<_> = a.iter().collect();
    assert_eq!(result.len(), expected.len());
    for &x in &expected {
        assert!(result.contains(&&x));
    }
}

#[test]
fn test_bitxor_assign_with_empty_rhs() {
    let mut a: HashSet<i32> = vec![1, 2, 3].into_iter().collect();
    let b: HashSet<i32> = HashSet::new();

    a ^= &b;

    let expected = [1, 2, 3];
    let result: Vec<_> = a.iter().collect();
    assert_eq!(result.len(), expected.len());
    for &x in &expected {
        assert!(result.contains(&&x));
    }
}

#[test]
fn test_bitxor_assign_with_empty_self() {
    let mut a: HashSet<i32> = HashSet::new();
    let b: HashSet<i32> = vec![3, 4, 5].into_iter().collect();

    a ^= &b;

    let expected = [3, 4, 5];
    let result: Vec<_> = a.iter().collect();
    assert_eq!(result.len(), expected.len());
    for &x in &expected {
        assert!(result.contains(&&x));
    }
}

