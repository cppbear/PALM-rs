// Answer 0

#[test]
fn test_decimal_length9_case_1() {
    let v: u32 = 10000000; // v is exactly 10000000
    let result = decimal_length9(v);
    assert_eq!(result, 8);
}

#[test]
fn test_decimal_length9_case_2() {
    let v: u32 = 99999999; // v is below 1000000000 and above 10000000
    let result = decimal_length9(v);
    assert_eq!(result, 8);
}

#[test]
fn test_decimal_length9_boundary_case() {
    let v: u32 = 1000000; // v is exactly 1000000
    let result = decimal_length9(v);
    assert_eq!(result, 7);
}

