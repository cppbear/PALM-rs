// Answer 0

#[test]
fn test_log2_pow5_zero() {
    assert_eq!(log2_pow5(0), 0);
}

#[test]
fn test_log2_pow5_small_value() {
    assert_eq!(log2_pow5(1), 0);
    assert_eq!(log2_pow5(2), 1);
    assert_eq!(log2_pow5(3), 1);
}

#[test]
fn test_log2_pow5_boundary_values() {
    assert_eq!(log2_pow5(3528), 879);
}

#[test]
#[should_panic]
fn test_log2_pow5_negative_value() {
    log2_pow5(-1);
}

#[test]
#[should_panic]
fn test_log2_pow5_exceeding_upper_bound() {
    log2_pow5(3529);
}

