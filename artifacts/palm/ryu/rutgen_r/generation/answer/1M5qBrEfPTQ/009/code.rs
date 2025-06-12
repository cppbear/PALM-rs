// Answer 0

#[test]
fn test_decimal_length17_case_1() {
    let v: u64 = 100000000;
    let result = decimal_length17(v);
    assert_eq!(result, 9);
}

#[test]
fn test_decimal_length17_case_2() {
    let v: u64 = 99999999;
    let result = decimal_length17(v);
    assert_eq!(result, 8);
}

#[test]
fn test_decimal_length17_case_3() {
    let v: u64 = 123456789;
    let result = decimal_length17(v);
    assert_eq!(result, 9);
}

#[test]
fn test_decimal_length17_case_4() {
    let v: u64 = 1000000000;
    let result = decimal_length17(v);
    assert_eq!(result, 10);
}

#[test]
fn test_decimal_length17_case_5() {
    let v: u64 = 99999999999999999; // Edge case (17-digit number)
    let result = decimal_length17(v);
    assert_eq!(result, 17);
}

