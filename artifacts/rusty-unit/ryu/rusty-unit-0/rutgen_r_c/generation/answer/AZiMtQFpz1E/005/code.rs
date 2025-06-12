// Answer 0

#[test]
fn test_decimal_length9_boundary_case() {
    let v = 10000;
    let result = decimal_length9(v);
    assert_eq!(result, 5);
}

#[test]
fn test_decimal_length9_large_case() {
    let v = 999999999; // Closest to the panic limit but still valid
    let result = decimal_length9(v);
    assert_eq!(result, 9);
}

#[test]
fn test_decimal_length9_small_case() {
    let v = 9; // Testing the smallest possible value
    let result = decimal_length9(v);
    assert_eq!(result, 1);
}

#[test]
fn test_decimal_length9_mid_case() {
    let v = 999; // Testing just below 1000
    let result = decimal_length9(v);
    assert_eq!(result, 3);
}

