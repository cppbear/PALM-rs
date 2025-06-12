// Answer 0

#[test]
fn test_to_raw_capacity_zero() {
    let n = 0;
    let _ = to_raw_capacity(n);
}

#[test]
fn test_to_raw_capacity_one() {
    let n = 1;
    let _ = to_raw_capacity(n);
}

#[test]
fn test_to_raw_capacity_two() {
    let n = 2;
    let _ = to_raw_capacity(n);
}

#[test]
fn test_to_raw_capacity_three() {
    let n = 3;
    let _ = to_raw_capacity(n);
}

#[test]
fn test_to_raw_capacity_four() {
    let n = 4;
    let _ = to_raw_capacity(n);
}

#[should_panic]
fn test_to_raw_capacity_overflow() {
    let n = usize::MAX;
    let _ = to_raw_capacity(n);
}

