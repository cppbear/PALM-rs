// Answer 0

#[test]
fn test_log10_pow2_lower_bound() {
    assert_eq!(log10_pow2(0), 0);
}

#[test]
fn test_log10_pow2_mid_range() {
    assert_eq!(log10_pow2(100), 443);
}

#[test]
fn test_log10_pow2_upper_bound() {
    assert_eq!(log10_pow2(1650), 78913);
}

#[should_panic]
fn test_log10_pow2_below_lower_bound() {
    let _ = log10_pow2(-1); // Should panic due to debug_assert
}

#[should_panic]
fn test_log10_pow2_above_upper_bound() {
    let _ = log10_pow2(1651); // Should panic due to debug_assert
}

