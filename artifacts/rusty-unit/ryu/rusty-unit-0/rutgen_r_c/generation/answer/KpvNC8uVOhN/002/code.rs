// Answer 0

#[test]
fn test_pow5bits_lower_bound() {
    let result = pow5bits(0);
    assert_eq!(result, 1);
}

#[test]
#[should_panic]
fn test_pow5bits_upper_bound() {
    let _ = pow5bits(3529); // This should trigger a panic due to the assert
}

