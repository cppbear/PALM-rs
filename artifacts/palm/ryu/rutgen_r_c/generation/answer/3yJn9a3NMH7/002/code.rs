// Answer 0

#[test]
fn test_log2_pow5_lower_bound() {
    let e: i32 = 0;
    let result = log2_pow5(e);
    assert_eq!(result, 0);
}

#[test]
#[should_panic]
fn test_log2_pow5_upper_bound_exceed() {
    let e: i32 = 3529;
    let _result = log2_pow5(e); // This should trigger a panic due to the upper boundary check
}

#[test]
fn test_log2_pow5_upper_bound_valid() {
    let e: i32 = 3528;
    let result = log2_pow5(e);
    assert_eq!(result, ((3528u32 * 1217359) >> 19) as i32);
}

