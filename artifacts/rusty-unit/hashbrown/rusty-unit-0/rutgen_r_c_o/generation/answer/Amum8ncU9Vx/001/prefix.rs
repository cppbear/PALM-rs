// Answer 0

#[test]
fn test_bitxor_assign_with_nonexistent_items() {
    let mut a: HashSet<i32> = vec![1, 2].into_iter().collect();
    let b: HashSet<i32> = vec![3, 4, 5].into_iter().collect();
    a ^= &b;
}

#[test]
fn test_bitxor_assign_with_existing_and_nonexistent_items() {
    let mut a: HashSet<i32> = vec![1, 2, 3].into_iter().collect();
    let b: HashSet<i32> = vec![3, 4, 5].into_iter().collect();
    a ^= &b;
}

#[test]
fn test_bitxor_assign_empty_rhs() {
    let mut a: HashSet<i32> = vec![1, 2, 3].into_iter().collect();
    let b: HashSet<i32> = HashSet::new();
    a ^= &b;
}

#[test]
fn test_bitxor_assign_with_all_existing_items() {
    let mut a: HashSet<i32> = vec![1, 2, 3].into_iter().collect();
    let b: HashSet<i32> = vec![1, 2, 3].into_iter().collect();
    a ^= &b;
}

#[test]
fn test_bitxor_assign_with_overlapping_items() {
    let mut a: HashSet<i32> = vec![1, 2, 3].into_iter().collect();
    let b: HashSet<i32> = vec![2, 3, 4].into_iter().collect();
    a ^= &b;
}

#[test]
fn test_bitxor_assign_with_maximum_capacity() {
    let mut a: HashSet<i32> = (1..=10).collect();
    let b: HashSet<i32> = (5..=15).collect();
    a ^= &b;
}

