// Answer 0

#[test]
fn test_to_raw_capacity_zero() {
    assert_eq!(to_raw_capacity(0), 0);
}

#[test]
fn test_to_raw_capacity_small_value() {
    assert_eq!(to_raw_capacity(3), 4);
}

#[test]
fn test_to_raw_capacity_large_value() {
    let value = usize::MAX / 4; // Using a value that will not cause overflow
    assert_eq!(to_raw_capacity(value), value + value / 3);
}

#[test]
#[should_panic(expected = "requested capacity")]
fn test_to_raw_capacity_overflow() {
    let _ = to_raw_capacity(usize::MAX);
}

