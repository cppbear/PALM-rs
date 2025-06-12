// Answer 0

#[test]
fn test_decimal_length9_case1() {
    let v = 1000;
    decimal_length9(v);
}

#[test]
fn test_decimal_length9_case2() {
    let v = 1001;
    decimal_length9(v);
}

#[test]
fn test_decimal_length9_case3() {
    let v = 9999;
    decimal_length9(v);
}

#[test]
fn test_decimal_length9_case4() {
    let v = 10000;
    decimal_length9(v);
}

#[test]
fn test_decimal_length9_case5() {
    let v = 999999;
    decimal_length9(v);
}

