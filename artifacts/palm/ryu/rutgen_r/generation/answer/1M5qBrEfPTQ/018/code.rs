// Answer 0

#[test]
#[should_panic] // This will trigger the debug assert since v >= 100000000000000000
fn test_decimal_length17_panic_high_bound() {
    let v: u64 = 100000000000000000;
    let _ = decimal_length17(v);
}

#[test]
fn test_decimal_length17_1_digit() {
    let v: u64 = 0;
    assert_eq!(decimal_length17(v), 1);
}

#[test]
fn test_decimal_length17_2_digits() {
    let v: u64 = 9;
    assert_eq!(decimal_length17(v), 1);

    let v: u64 = 10;
    assert_eq!(decimal_length17(v), 2);
}

#[test]
fn test_decimal_length17_3_digits() {
    let v: u64 = 99;
    assert_eq!(decimal_length17(v), 2);

    let v: u64 = 100;
    assert_eq!(decimal_length17(v), 3);
}

#[test]
fn test_decimal_length17_4_digits() {
    let v: u64 = 999;
    assert_eq!(decimal_length17(v), 3);

    let v: u64 = 1000;
    assert_eq!(decimal_length17(v), 4);
}

#[test]
fn test_decimal_length17_5_digits() {
    let v: u64 = 9999;
    assert_eq!(decimal_length17(v), 4);

    let v: u64 = 10000;
    assert_eq!(decimal_length17(v), 5);
}

#[test]
fn test_decimal_length17_6_digits() {
    let v: u64 = 99999;
    assert_eq!(decimal_length17(v), 5);

    let v: u64 = 100000;
    assert_eq!(decimal_length17(v), 6);
}

#[test]
fn test_decimal_length17_7_digits() {
    let v: u64 = 999999;
    assert_eq!(decimal_length17(v), 6);

    let v: u64 = 1000000;
    assert_eq!(decimal_length17(v), 7);
}

#[test]
fn test_decimal_length17_8_digits() {
    let v: u64 = 9999999;
    assert_eq!(decimal_length17(v), 7);
    
    let v: u64 = 10000000;
    assert_eq!(decimal_length17(v), 8);
}

#[test]
fn test_decimal_length17_9_digits() {
    let v: u64 = 99999999;
    assert_eq!(decimal_length17(v), 8);
    
    let v: u64 = 100000000;
    assert_eq!(decimal_length17(v), 9);
}

#[test]
fn test_decimal_length17_10_digits() {
    let v: u64 = 999999999;
    assert_eq!(decimal_length17(v), 9);
    
    let v: u64 = 1000000000;
    assert_eq!(decimal_length17(v), 10);
}

#[test]
fn test_decimal_length17_11_digits() {
    let v: u64 = 9999999999;
    assert_eq!(decimal_length17(v), 10);
    
    let v: u64 = 10000000000;
    assert_eq!(decimal_length17(v), 11);
}

#[test]
fn test_decimal_length17_12_digits() {
    let v: u64 = 99999999999;
    assert_eq!(decimal_length17(v), 11);
    
    let v: u64 = 100000000000;
    assert_eq!(decimal_length17(v), 12);
}

#[test]
fn test_decimal_length17_13_digits() {
    let v: u64 = 999999999999;
    assert_eq!(decimal_length17(v), 12);
    
    let v: u64 = 1000000000000;
    assert_eq!(decimal_length17(v), 13);
}

#[test]
fn test_decimal_length17_14_digits() {
    let v: u64 = 9999999999999;
    assert_eq!(decimal_length17(v), 13);
    
    let v: u64 = 10000000000000;
    assert_eq!(decimal_length17(v), 14);
}

#[test]
fn test_decimal_length17_15_digits() {
    let v: u64 = 99999999999999;
    assert_eq!(decimal_length17(v), 14);
    
    let v: u64 = 100000000000000;
    assert_eq!(decimal_length17(v), 15);
}

#[test]
fn test_decimal_length17_16_digits() {
    let v: u64 = 999999999999999;
    assert_eq!(decimal_length17(v), 15);
    
    let v: u64 = 1000000000000000;
    assert_eq!(decimal_length17(v), 16);
}

#[test]
fn test_decimal_length17_17_digits() {
    let v: u64 = 9999999999999999;
    assert_eq!(decimal_length17(v), 16);

    let v: u64 = 10000000000000000;
    assert_eq!(decimal_length17(v), 17);
}

