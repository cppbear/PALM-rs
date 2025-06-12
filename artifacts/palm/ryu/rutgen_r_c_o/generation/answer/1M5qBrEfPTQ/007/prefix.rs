// Answer 0

#[test]
fn test_decimal_length17_case_1() {
    let v: u64 = 10000000000;
    decimal_length17(v);
}

#[test]
fn test_decimal_length17_case_2() {
    let v: u64 = 9999999999;
    decimal_length17(v);
}

#[test]
fn test_decimal_length17_case_3() {
    let v: u64 = 1000000000;
    decimal_length17(v);
}

#[test]
fn test_decimal_length17_case_4() {
    let v: u64 = 999999999;
    decimal_length17(v);
}

#[test]
fn test_decimal_length17_case_5() {
    let v: u64 = 100000000;
    decimal_length17(v);
}

#[test]
fn test_decimal_length17_case_6() {
    let v: u64 = 99999999;
    decimal_length17(v);
}

#[test]
fn test_decimal_length17_case_7() {
    let v: u64 = 10000000;
    decimal_length17(v);
}

#[test]
fn test_decimal_length17_case_8() {
    let v: u64 = 9999999;
    decimal_length17(v);
}

#[test]
fn test_decimal_length17_case_9() {
    let v: u64 = 1000000;
    decimal_length17(v);
}

#[test]
fn test_decimal_length17_case_10() {
    let v: u64 = 999999;
    decimal_length17(v);
}

