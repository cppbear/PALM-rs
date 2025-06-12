// Answer 0

#[test]
fn test_decimal_length9_single_digit() {
    assert_eq!(decimal_length9(0), 1);
    assert_eq!(decimal_length9(5), 1);
    assert_eq!(decimal_length9(9), 1);
}

#[test]
fn test_decimal_length9_double_digit() {
    assert_eq!(decimal_length9(10), 2);
    assert_eq!(decimal_length9(99), 2);
}

#[test]
fn test_decimal_length9_triple_digit() {
    assert_eq!(decimal_length9(100), 3);
    assert_eq!(decimal_length9(999), 3);
}

#[test]
fn test_decimal_length9_quadruple_digit() {
    assert_eq!(decimal_length9(1000), 4);
    assert_eq!(decimal_length9(9999), 4);
}

#[test]
fn test_decimal_length9_quintuple_digit() {
    assert_eq!(decimal_length9(10000), 5);
    assert_eq!(decimal_length9(99999), 5);
}

#[test]
fn test_decimal_length9_sextuple_digit() {
    assert_eq!(decimal_length9(100000), 6);
    assert_eq!(decimal_length9(999999), 6);
}

#[test]
fn test_decimal_length9_septuple_digit() {
    assert_eq!(decimal_length9(1000000), 7);
    assert_eq!(decimal_length9(9999999), 7);
}

#[test]
fn test_decimal_length9_octuple_digit() {
    assert_eq!(decimal_length9(10000000), 8);
    assert_eq!(decimal_length9(99999999), 8);
}

#[test]
fn test_decimal_length9_nontuple_digit() {
    assert_eq!(decimal_length9(100000000), 9);
    assert_eq!(decimal_length9(999999999), 9);
}

#[should_panic]
fn test_decimal_length9_ten_digit() {
    decimal_length9(1000000000);  // should panic due to precondition
}

