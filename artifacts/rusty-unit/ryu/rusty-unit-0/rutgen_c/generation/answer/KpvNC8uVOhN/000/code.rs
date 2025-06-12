// Answer 0

#[test]
fn test_pow5bits_min_value() {
    let result = pow5bits(0);
    assert_eq!(result, 1);
}

#[test]
fn test_pow5bits_boundary_value() {
    let result = pow5bits(3528);
    assert_eq!(result, 100000);
}

#[test]
#[should_panic]
fn test_pow5bits_below_lower_bound() {
    let _ = pow5bits(-1);
}

#[test]
#[should_panic]
fn test_pow5bits_above_upper_bound() {
    let _ = pow5bits(3529);
}

#[test]
fn test_pow5bits_typical_value() {
    let result = pow5bits(10);
    assert_eq!(result, 482);
}

