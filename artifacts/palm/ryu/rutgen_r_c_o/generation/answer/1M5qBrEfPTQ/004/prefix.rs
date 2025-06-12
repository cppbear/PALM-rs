// Answer 0

#[test]
fn test_decimal_length17_case1() {
    let v: u64 = 10000000000000;
    decimal_length17(v);
}

#[test]
fn test_decimal_length17_case2() {
    let v: u64 = 10000000000001;
    decimal_length17(v);
}

#[test]
fn test_decimal_length17_case3() {
    let v: u64 = 99999999999999;
    decimal_length17(v);
}

#[test]
fn test_decimal_length17_case4() {
    let v: u64 = 99999999999990;
    decimal_length17(v);
}

#[test]
fn test_decimal_length17_case5() {
    let v: u64 = 10000000000000 + 1;
    decimal_length17(v);
}

