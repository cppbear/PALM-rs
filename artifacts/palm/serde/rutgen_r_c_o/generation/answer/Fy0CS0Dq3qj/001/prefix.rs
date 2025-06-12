// Answer 0

#[test]
fn test_nop_reserve_zero_capacity() {
    let seq: Vec<i32> = Vec::new();
    nop_reserve(seq, 0);
}

#[test]
fn test_nop_reserve_small_capacity() {
    let seq: Vec<i32> = Vec::new();
    nop_reserve(seq, 10);
}

#[test]
fn test_nop_reserve_large_capacity() {
    let seq: Vec<i32> = Vec::new();
    nop_reserve(seq, usize::MAX);
}

#[test]
fn test_nop_reserve_large_capacity_with_limit() {
    let seq: Vec<i32> = Vec::new();
    nop_reserve(seq, 1_000_000);
}

