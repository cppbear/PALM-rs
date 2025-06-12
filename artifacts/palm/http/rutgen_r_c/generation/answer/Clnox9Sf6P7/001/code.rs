// Answer 0

#[test]
fn test_usable_capacity_zero() {
    let capacity = 0;
    let expected = 0; // 0 - 0/4 = 0
    assert_eq!(usable_capacity(capacity), expected);
}

#[test]
fn test_usable_capacity_small_positive() {
    let capacity = 1;
    let expected = 0; // 1 - 1/4 = 0
    assert_eq!(usable_capacity(capacity), expected);
}

#[test]
fn test_usable_capacity_small_negative() {
    let capacity = 4;
    let expected = 3; // 4 - 4/4 = 3
    assert_eq!(usable_capacity(capacity), expected);
}

#[test]
fn test_usable_capacity_large() {
    let capacity = 16;
    let expected = 12; // 16 - 16/4 = 12
    assert_eq!(usable_capacity(capacity), expected);
}

#[test]
fn test_usable_capacity_boundary() {
    let capacity = 64;
    let expected = 48; // 64 - 64/4 = 48
    assert_eq!(usable_capacity(capacity), expected);
}

#[test]
fn test_usable_capacity_max() {
    let capacity = usize::MAX;
    let expected = usize::MAX - (usize::MAX / 4); // Handling maximum capacity
    assert_eq!(usable_capacity(capacity), expected);
}

