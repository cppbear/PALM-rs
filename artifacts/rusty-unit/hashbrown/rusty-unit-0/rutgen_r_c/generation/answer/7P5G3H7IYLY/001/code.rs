// Answer 0

#[test]
fn test_h1_zero() {
    let input: u64 = 0;
    let expected: usize = 0;
    let result = h1(input);
    assert_eq!(result, expected);
}

#[test]
fn test_h1_positive() {
    let input: u64 = 12345;
    let expected: usize = 12345;
    let result = h1(input);
    assert_eq!(result, expected);
}

#[test]
fn test_h1_max_u64() {
    let input: u64 = u64::MAX;
    let expected: usize = u64::MAX as usize; // Longevity of value, expected to not panic 
    let result = h1(input);
    assert_eq!(result, expected);
}

#[test]
fn test_h1_boundary_minus_one() {
    let input: u64 = u64::MAX - 1;
    let expected: usize = (u64::MAX - 1) as usize;
    let result = h1(input);
    assert_eq!(result, expected);
}

