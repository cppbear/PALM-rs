// Answer 0

#[test]
fn test_log2_pow5_bound_zero() {
    let e = 0;
    let result = log2_pow5(e);
    assert_eq!(result, 0);
}

#[test]
fn test_log2_pow5_bound_max() {
    let e = 3528;
    let result = log2_pow5(e);
    assert_eq!(result, 3511); // Expected value calculated as ((3528 as u32 * 1217359) >> 19) as i32
}

