// Answer 0

#[test]
fn test_decimal_length9_case_8() {
    let v = 10000000;
    let result = decimal_length9(v);
}

#[test]
fn test_decimal_length9_case_8_boundary() {
    let v = 99999999;
    let result = decimal_length9(v);
}

