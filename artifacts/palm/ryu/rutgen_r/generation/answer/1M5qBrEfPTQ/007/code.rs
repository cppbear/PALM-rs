// Answer 0

#[test]
fn test_decimal_length17_edge_case_1() {
    let v: u64 = 10000000000; // v is >= 10000000000 and < 100000000000000000
    let result = decimal_length17(v);
    assert_eq!(result, 11);
}

#[test]
fn test_decimal_length17_edge_case_2() {
    let v: u64 = 9999999999; // v is < 10000000000 and >= 1000000000
    let result = decimal_length17(v);
    assert_eq!(result, 10);
}

#[test]
fn test_decimal_length17_edge_case_3() {
    let v: u64 = 999999999; // v < 1000000000 and >= 100000000
    let result = decimal_length17(v);
    assert_eq!(result, 9);
}

#[test]
fn test_decimal_length17_edge_case_4() {
    let v: u64 = 100; // v >= 100 and < 1000
    let result = decimal_length17(v);
    assert_eq!(result, 3);
}

#[test]
fn test_decimal_length17_edge_case_5() {
    let v: u64 = 1; // v is < 10
    let result = decimal_length17(v);
    assert_eq!(result, 1);
}

