// Answer 0

#[test]
fn test_nop_reserve_with_zero() {
    let seq = Vec::<i32>::new();
    nop_reserve(seq, 0);
}

#[test]
fn test_nop_reserve_with_non_zero() {
    let seq = Vec::<i32>::new();
    nop_reserve(seq, 10);
}

