// Answer 0

#[test]
fn test_resize_capacity_equals_num_insts_zero() {
    let mut threads = Threads::new();
    threads.resize(0, 0);
}

#[test]
fn test_resize_capacity_equals_num_insts_non_zero() {
    let mut threads = Threads::new();
    threads.resize(0, 1);
}

#[test]
fn test_resize_capacity_equals_num_insts_non_zero_multiple_ncaps() {
    let mut threads = Threads::new();
    threads.resize(0, 2);
}

#[test]
fn test_resize_capacity_equals_num_insts_large_ncaps() {
    let mut threads = Threads::new();
    threads.resize(0, 5);
}

#[test]
fn test_resize_capacity_equals_num_insts_edge_case_high_ncaps() {
    let mut threads = Threads::new();
    threads.resize(0, usize::MAX);
}

