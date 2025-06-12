// Answer 0

#[test]
fn test_decimal_length17_boundary_case_10() {
    let v: u64 = 10;
    assert_eq!(decimal_length17(v), 2);
}

#[test]
fn test_decimal_length17_boundary_case_1() {
    let v: u64 = 1;
    assert_eq!(decimal_length17(v), 1);
}

#[test]
fn test_decimal_length17_boundary_case_99() {
    let v: u64 = 99;
    assert_eq!(decimal_length17(v), 2);
}

#[test]
fn test_decimal_length17_boundary_case_100() {
    let v: u64 = 100;
    assert_eq!(decimal_length17(v), 3);
}

#[test]
fn test_decimal_length17_boundary_case_1000() {
    let v: u64 = 1000;
    assert_eq!(decimal_length17(v), 4);
}

#[test]
fn test_decimal_length17_boundary_case_99999999999999999() {
    let v: u64 = 99999999999999999;
    assert_eq!(decimal_length17(v), 17);
}

#[test]
#[should_panic]
fn test_decimal_length17_boundary_case_100000000000000000() {
    let v: u64 = 100000000000000000;
    decimal_length17(v);
}

