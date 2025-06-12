// Answer 0

#[test]
fn test_decimal_length17_case_with_boundary_value() {
    let value: u64 = 10000; // v >= 10000 is true
    let expected_length: u32 = 5; // Expected return value

    let result = decimal_length17(value);
    assert_eq!(result, expected_length);
}

#[test]
fn test_decimal_length17_case_with_minimum_value() {
    let value: u64 = 1;
    let expected_length: u32 = 1;

    let result = decimal_length17(value);
    assert_eq!(result, expected_length);
}

#[test]
fn test_decimal_length17_case_with_small_value() {
    let value: u64 = 9; 
    let expected_length: u32 = 1; 

    let result = decimal_length17(value);
    assert_eq!(result, expected_length);
}

#[test]
fn test_decimal_length17_case_with_two_digits() {
    let value: u64 = 10; 
    let expected_length: u32 = 2; 

    let result = decimal_length17(value);
    assert_eq!(result, expected_length);
}

#[test]
fn test_decimal_length17_case_with_three_digits() {
    let value: u64 = 100; 
    let expected_length: u32 = 3; 

    let result = decimal_length17(value);
    assert_eq!(result, expected_length);
}

#[test]
fn test_decimal_length17_case_with_four_digits() {
    let value: u64 = 1000; 
    let expected_length: u32 = 4; 

    let result = decimal_length17(value);
    assert_eq!(result, expected_length);
}

