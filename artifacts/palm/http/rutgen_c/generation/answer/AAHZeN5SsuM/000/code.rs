// Answer 0

#[test]
fn test_to_raw_capacity_zero() {
    let result = to_raw_capacity(0);
    assert_eq!(result, 0);
}

#[test]
fn test_to_raw_capacity_small_number() {
    let result = to_raw_capacity(3);
    assert_eq!(result, 4); // 3 + (3 / 3) = 3 + 1 = 4
}

#[test]
fn test_to_raw_capacity_boundary_case() {
    let result = to_raw_capacity(u16::MAX as usize);
    assert!(result > u16::MAX as usize); // This checks the expected capacity exceeds u16 max
}

#[test]
#[should_panic(expected = "requested capacity 18446744073709551615 too large: overflow while converting to raw capacity")]
fn test_to_raw_capacity_overflow() {
    let _ = to_raw_capacity(usize::MAX);
}

