// Answer 0

#[test]
fn test_decimal_length9_case_1() {
    let v = 1000000;
    let result = decimal_length9(v);
    assert_eq!(result, 7);
}

#[test]
fn test_decimal_length9_case_2() {
    let v = 999999;
    let result = decimal_length9(v);
    assert_eq!(result, 6);
}

#[test]
fn test_decimal_length9_case_3() {
    let v = 99999;
    let result = decimal_length9(v);
    assert_eq!(result, 5);
}

#[test]
fn test_decimal_length9_case_4() {
    let v = 9999;
    let result = decimal_length9(v);
    assert_eq!(result, 4);
}

#[test]
fn test_decimal_length9_case_5() {
    let v = 999;
    let result = decimal_length9(v);
    assert_eq!(result, 3);
}

#[test]
fn test_decimal_length9_case_6() {
    let v = 99;
    let result = decimal_length9(v);
    assert_eq!(result, 2);
}

#[test]
fn test_decimal_length9_case_7() {
    let v = 9;
    let result = decimal_length9(v);
    assert_eq!(result, 1);
}

