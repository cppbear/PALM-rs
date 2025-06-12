// Answer 0

#[test]
fn test_log10_pow5_zero() {
    let result = log10_pow5(0);
    assert_eq!(result, 0);
}

#[test]
#[should_panic]
fn test_log10_pow5_boundary_high() {
    let _ = log10_pow5(2621);
}

#[test]
fn test_log10_pow5_low_boundary() {
    let result = log10_pow5(2620);
    assert_eq!(result, 732923);
}

