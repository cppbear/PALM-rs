// Answer 0

#[test]
fn test_decimal_length9_zero() {
    decimal_length9(0);
}

#[test]
fn test_decimal_length9_one() {
    decimal_length9(1);
}

#[test]
fn test_decimal_length9_ten() {
    decimal_length9(10);
}

#[test]
fn test_decimal_length9_hundred() {
    decimal_length9(100);
}

#[test]
fn test_decimal_length9_thousand() {
    decimal_length9(1000);
}

#[test]
fn test_decimal_length9_ten_thousand() {
    decimal_length9(10000);
}

#[test]
fn test_decimal_length9_hundred_thousand() {
    decimal_length9(100000);
}

#[test]
fn test_decimal_length9_million() {
    decimal_length9(1000000);
}

#[test]
fn test_decimal_length9_ten_million() {
    decimal_length9(10000000);
}

#[test]
fn test_decimal_length9_hundred_million() {
    decimal_length9(100000000);
}

#[test]
fn test_decimal_length9_nine_hundred_ninety_nine_million_nine_hundred_ninety_nine_thousand_nine_hundred_ninety_nine() {
    decimal_length9(999999999);
}

