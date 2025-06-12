// Answer 0

#[test]
fn test_log10_pow2_lower_bound() {
    let e = 0;
    let result = log10_pow2(e);
    assert_eq!(result, 0);
}

#[test]
#[should_panic]
fn test_log10_pow2_upper_bound_exceed() {
    let e = 1651;
    let _result = log10_pow2(e);
}

#[test]
fn test_log10_pow2_middle_bound() {
    let e = 825; // Midpoint for e within valid range
    let result = log10_pow2(e);
    assert_eq!(result, (e as u32 * 78913) >> 18); // Validate computation
}

#[test]
fn test_log10_pow2_near_upper_bound() {
    let e = 1650; // Upper valid bound
    let result = log10_pow2(e);
    assert_eq!(result, (e as u32 * 78913) >> 18); // Validate computation
}

