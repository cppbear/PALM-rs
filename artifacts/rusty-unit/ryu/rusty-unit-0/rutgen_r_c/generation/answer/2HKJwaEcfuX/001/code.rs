// Answer 0

#[test]
fn test_log10_pow2_lower_bound() {
    let e: i32 = 0;
    let expected_result: u32 = (e as u32 * 78913) >> 18;
    let result = log10_pow2(e);
    assert_eq!(result, expected_result);
}

#[test]
fn test_log10_pow2_upper_bound() {
    let e: i32 = 1650;
    let expected_result: u32 = (e as u32 * 78913) >> 18;
    let result = log10_pow2(e);
    assert_eq!(result, expected_result);
}

#[test]
#[should_panic]
fn test_log10_pow2_beyond_upper_bound() {
    let e: i32 = 1651;
    let _result = log10_pow2(e);
}

#[test]
#[should_panic]
fn test_log10_pow2_negative_input() {
    let e: i32 = -1;
    let _result = log10_pow2(e);
}

