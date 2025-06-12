// Answer 0

#[test]
fn test_div5_zero() {
    let result = div5(0);
    assert_eq!(result, 0);
}

#[test]
fn test_div5_small_number() {
    let result = div5(4);
    assert_eq!(result, 0);
}

#[test]
fn test_div5_exact_division() {
    let result = div5(10);
    assert_eq!(result, 2);
}

#[test]
fn test_div5_large_number() {
    let result = div5(1_000_000);
    assert_eq!(result, 200_000);
}

#[test]
fn test_div5_large_number_just_below_multiple_of_five() {
    let result = div5(999_999);
    assert_eq!(result, 199_999);
}

#[test]
fn test_div5_large_number_exactly_multiple_of_five() {
    let result = div5(1_000_005);
    assert_eq!(result, 200_001);
}

#[test]
fn test_div5_maximum_u64() {
    let result = div5(u64::MAX);
    assert_eq!(result, u64::MAX / 5);
}

