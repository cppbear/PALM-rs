// Answer 0

#[test]
#[should_panic]
fn test_log10_pow2_negative_input() {
    let e: i32 = -1;
    let result = log10_pow2(e);
}

#[test]
fn test_log10_pow2_zero_input() {
    let e: i32 = 0;
    let result = log10_pow2(e);
    assert_eq!(result, 0);
}

#[test]
fn test_log10_pow2_upper_boundary_input() {
    let e: i32 = 1650;
    let result = log10_pow2(e);
    assert_eq!(result, 78913);
}

#[test]
fn test_log10_pow2_mid_range_input() {
    let e: i32 = 825;
    let result = log10_pow2(e);
    assert_eq!(result, 39456);
}

