// Answer 0

#[test]
fn test_log10_pow2_lower_bound() {
    assert_eq!(log10_pow2(0), 0);
}

#[test]
fn test_log10_pow2_mid_range() {
    assert_eq!(log10_pow2(1000), 439);
}

#[test]
fn test_log10_pow2_upper_bound() {
    assert_eq!(log10_pow2(1650), 7888);
}

#[test]
#[should_panic]
fn test_log10_pow2_negative() {
    log10_pow2(-1);
}

#[test]
#[should_panic]
fn test_log10_pow2_above_upper_bound() {
    log10_pow2(1651);
}

