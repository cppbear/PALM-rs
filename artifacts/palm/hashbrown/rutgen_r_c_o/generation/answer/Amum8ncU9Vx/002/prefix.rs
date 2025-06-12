// Answer 0

#[test]
fn test_bitxor_assign_some_overlap() {
    let mut a: HashSet<i32> = (0..50).collect();
    let b: HashSet<i32> = (25..75).collect();
    a ^= &b;
}

#[test]
fn test_bitxor_assign_all_overlap() {
    let mut a: HashSet<i32> = (0..100).collect();
    let b: HashSet<i32> = (0..100).collect();
    a ^= &b;
}

#[test]
fn test_bitxor_assign_no_overlap() {
    let mut a: HashSet<i32> = (0..50).collect();
    let b: HashSet<i32> = (50..100).collect();
    a ^= &b;
}

#[test]
fn test_bitxor_assign_empty_rhs() {
    let mut a: HashSet<i32> = (10..20).collect();
    let b: HashSet<i32> = HashSet::new();
    a ^= &b;
}

#[test]
fn test_bitxor_assign_empty_self() {
    let mut a: HashSet<i32> = HashSet::new();
    let b: HashSet<i32> = (0..10).collect();
    a ^= &b;
}

#[test]
fn test_bitxor_assign_edge_cases() {
    let mut a: HashSet<i32> = (1..10).collect();
    let b: HashSet<i32> = (9..20).collect();
    a ^= &b;
}

