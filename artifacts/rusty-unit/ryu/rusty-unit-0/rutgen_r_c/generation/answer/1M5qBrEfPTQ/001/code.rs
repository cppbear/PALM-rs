// Answer 0

#[test]
fn test_decimal_length17_boundary_17_digits() {
    let input: u64 = 10000000000000000; // v >= 10000000000000000
    let expected_output: u32 = 17;

    let output = decimal_length17(input);
    assert_eq!(output, expected_output);
}

#[test]
fn test_decimal_length17_boundary_16_digits() {
    let input: u64 = 9999999999999999; // v < 10000000000000000
    let expected_output: u32 = 16;

    let output = decimal_length17(input);
    assert_eq!(output, expected_output);
}

#[test]
fn test_decimal_length17_boundary_15_digits() {
    let input: u64 = 999999999999999; // v < 1000000000000000
    let expected_output: u32 = 15;

    let output = decimal_length17(input);
    assert_eq!(output, expected_output);
}

#[test]
fn test_decimal_length17_boundary_14_digits() {
    let input: u64 = 99999999999999; // v < 10000000000000
    let expected_output: u32 = 14;

    let output = decimal_length17(input);
    assert_eq!(output, expected_output);
}

#[test]
fn test_decimal_length17_minimum_value() {
    let input: u64 = 0; // Minimum case
    let expected_output: u32 = 1;

    let output = decimal_length17(input);
    assert_eq!(output, expected_output);
}

