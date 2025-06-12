// Answer 0

#[test]
fn test_decimal_length17_boundary_values() {
    assert_eq!(decimal_length17(0), 1);
    assert_eq!(decimal_length17(9), 1);
    assert_eq!(decimal_length17(10), 2);
    assert_eq!(decimal_length17(99), 2);
    assert_eq!(decimal_length17(100), 3);
    assert_eq!(decimal_length17(999), 3);
    assert_eq!(decimal_length17(1000), 4);
    assert_eq!(decimal_length17(9999), 4);
    assert_eq!(decimal_length17(10000), 5);
    assert_eq!(decimal_length17(99999), 5);
    assert_eq!(decimal_length17(100000), 6);
    assert_eq!(decimal_length17(999999), 6);
    assert_eq!(decimal_length17(1000000), 7);
    assert_eq!(decimal_length17(9999999), 7);
    assert_eq!(decimal_length17(10000000), 8);
    assert_eq!(decimal_length17(99999999), 8);
    assert_eq!(decimal_length17(100000000), 9);
    assert_eq!(decimal_length17(999999999), 9);
    assert_eq!(decimal_length17(1000000000), 10);
    assert_eq!(decimal_length17(9999999999), 10);
    assert_eq!(decimal_length17(10000000000), 11);
    assert_eq!(decimal_length17(99999999999), 11);
    assert_eq!(decimal_length17(100000000000), 12);
    assert_eq!(decimal_length17(999999999999), 12);
    assert_eq!(decimal_length17(1000000000000), 13);
    assert_eq!(decimal_length17(9999999999999), 13);
    assert_eq!(decimal_length17(10000000000000), 14);
    assert_eq!(decimal_length17(99999999999999), 14);
    assert_eq!(decimal_length17(100000000000000), 15);
    assert_eq!(decimal_length17(999999999999999), 15);
    assert_eq!(decimal_length17(1000000000000000), 16);
    assert_eq!(decimal_length17(9999999999999999), 16);
    assert_eq!(decimal_length17(10000000000000000), 17);
}

#[test]
#[should_panic]
fn test_decimal_length17_out_of_bounds() {
    decimal_length17(100000000000000000);
}

#[test]
fn test_decimal_length17_large_values() {
    assert_eq!(decimal_length17(99999999999999999), 17);
}

