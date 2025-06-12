// Answer 0

#[test]
fn test_resize_increase_size() {
    let mut threads = Threads::new();
    threads.resize(10, 5);
}

#[test]
fn test_resize_increase_size_with_zero_ncaps() {
    let mut threads = Threads::new();
    threads.resize(15, 0);
}

#[test]
fn test_resize_increase_size_max_caps() {
    let mut threads = Threads::new();
    threads.resize(20, 500);
}

#[test]
fn test_resize_with_same_capacity() {
    let mut threads = Threads::new();
    threads.set = SparseSet::new(10);
    threads.resize(10, 5);
}

#[test]
fn test_resize_increase_size_at_upper_bound() {
    let mut threads = Threads::new();
    threads.resize(1000, 250);
}

#[test]
fn test_resize_with_varying_ncaps() {
    let mut threads = Threads::new();
    threads.resize(100, 10);
    threads.resize(50, 20);
}

