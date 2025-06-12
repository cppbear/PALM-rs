// Answer 0

#[test]
fn test_to_raw_capacity_zero() {
    let result = to_raw_capacity(0);
    assert_eq!(result, 0);
}

#[test]
fn test_to_raw_capacity_one() {
    let result = to_raw_capacity(1);
    assert_eq!(result, 1);
}

#[test]
fn test_to_raw_capacity_two() {
    let result = to_raw_capacity(2);
    assert_eq!(result, 2);
}

#[test]
fn test_to_raw_capacity_three() {
    let result = to_raw_capacity(3);
    assert_eq!(result, 4);
}

#[test]
fn test_to_raw_capacity_twelve() {
    let result = to_raw_capacity(12);
    assert_eq!(result, 16);
}

#[test]
#[should_panic(expected = "requested capacity 18446744073709551615 too large: overflow while converting to raw capacity")]
fn test_to_raw_capacity_max() {
    to_raw_capacity(usize::MAX);
}

