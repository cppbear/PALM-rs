// Answer 0

#[test]
fn test_log10_pow5_lower_boundary() {
    let result = log10_pow5(0);
    assert_eq!(result, 0);
}

#[test]
fn test_log10_pow5_upper_boundary() {
    let result = log10_pow5(2620);
    assert_eq!(result, 174818971);
}

#[test]
fn test_log10_pow5_valid_mid_range() {
    let result = log10_pow5(1310);
    assert_eq!(result, 97522);
}

#[test]
#[should_panic]
fn test_log10_pow5_below_lower_boundary() {
    log10_pow5(-1);
}

#[test]
#[should_panic]
fn test_log10_pow5_above_upper_boundary() {
    log10_pow5(2621);
}

