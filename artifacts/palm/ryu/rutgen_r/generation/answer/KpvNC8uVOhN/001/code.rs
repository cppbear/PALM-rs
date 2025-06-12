// Answer 0

#[test]
fn test_pow5bits_zero() {
    let e: i32 = 0;
    let result = pow5bits(e);
    assert_eq!(result, 1);
}

#[test]
fn test_pow5bits_max() {
    let e: i32 = 3528;
    let result = pow5bits(e);
    assert_eq!(result, 253130586);
}

#[test]
#[should_panic]
fn test_pow5bits_negative() {
    let e: i32 = -1; // This should panic due to the debug assertion
    let _ = pow5bits(e);
}

#[test]
#[should_panic]
fn test_pow5bits_overflow() {
    let e: i32 = 3529; // This should panic due to the debug assertion
    let _ = pow5bits(e);
}

