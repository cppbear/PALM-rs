// Answer 0

#[test]
fn test_sub_assign_empty_sets() {
    let mut a: HashSet<i32> = HashSet::new();
    let b: HashSet<i32> = HashSet::new();
    a -= &b;
}

#[test]
fn test_sub_assign_single_element_sets_equal() {
    let mut a: HashSet<i32> = vec![1].into_iter().collect();
    let b: HashSet<i32> = vec![1].into_iter().collect();
    a -= &b;
}

#[test]
fn test_sub_assign_two_element_sets_equal() {
    let mut a: HashSet<i32> = vec![1, 2].into_iter().collect();
    let b: HashSet<i32> = vec![1, 2].into_iter().collect();
    a -= &b;
}

#[test]
fn test_sub_assign_large_equal_sets() {
    let mut a: HashSet<i32> = (0..1000).collect();
    let b: HashSet<i32> = (0..1000).collect();
    a -= &b;
}

#[test]
fn test_sub_assign_large_unequal_sets() {
    let mut a: HashSet<i32> = (0..1000).collect();
    let b: HashSet<i32> = (0..500).collect();
    a -= &b;
}

