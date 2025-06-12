// Answer 0

#[test]
fn test_bitand_assign_non_empty_sets() {
    let mut a: HashSet<i32> = (1..=50).collect();
    let b: HashSet<i32> = (25..=75).collect();
    a &= &b;
}

#[test]
fn test_bitand_assign_empty_rhs() {
    let mut a: HashSet<i32> = (1..=50).collect();
    let b: HashSet<i32> = HashSet::new();
    a &= &b;
}

#[test]
fn test_bitand_assign_empty_self() {
    let mut a: HashSet<i32> = HashSet::new();
    let b: HashSet<i32> = (1..=50).collect();
    a &= &b;
}

#[test]
fn test_bitand_assign_disjoint_sets() {
    let mut a: HashSet<i32> = (1..=50).collect();
    let b: HashSet<i32> = (51..=100).collect();
    a &= &b;
}

#[test]
fn test_bitand_assign_identical_sets() {
    let mut a: HashSet<i32> = (1..=50).collect();
    let b: HashSet<i32> = (1..=50).collect();
    a &= &b;
}

#[test]
fn test_bitand_assign_edge_case_single_element() {
    let mut a: HashSet<i32> = (1..=10).collect();
    let b: HashSet<i32> = (5..=5).collect();
    a &= &b;
}

#[test]
fn test_bitand_assign_large_sets() {
    let mut a: HashSet<i32> = (1..=100).collect();
    let b: HashSet<i32> = (51..=150).collect();
    a &= &b;
}

#[test]
fn test_bitand_assign_rhs_larger_than_self() {
    let mut a: HashSet<i32> = (1..=50).collect();
    let b: HashSet<i32> = (1..=100).collect();
    a &= &b;
}

#[test]
fn test_bitand_assign_multiple_operations() {
    let mut a: HashSet<i32> = (1..=100).collect();
    let b: HashSet<i32> = (1..=50).collect();
    let c: HashSet<i32> = (25..=75).collect();
    a &= &b;
    a &= &c;
}

