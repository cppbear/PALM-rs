// Answer 0

#[test]
fn test_log10_pow5_zero() {
    assert_eq!(log10_pow5(0), 0);
}

#[test]
fn test_log10_pow5_small() {
    assert_eq!(log10_pow5(1), 35);
    assert_eq!(log10_pow5(2), 70);
    assert_eq!(log10_pow5(3), 105);
}

#[test]
fn test_log10_pow5_boundary_low() {
    assert_eq!(log10_pow5(2620), 732923);
}

#[should_panic]
fn test_log10_pow5_negative() {
    log10_pow5(-1);
}

#[should_panic]
fn test_log10_pow5_too_large() {
    log10_pow5(2621);
}

