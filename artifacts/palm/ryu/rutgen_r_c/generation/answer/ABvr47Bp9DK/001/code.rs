// Answer 0

#[test]
fn test_log10_pow5_zero() {
    let e: i32 = 0;
    let result = log10_pow5(e);
    assert_eq!(result, 0);
}

#[test]
fn test_log10_pow5_max() {
    let e: i32 = 2620;
    let result = log10_pow5(e);
    assert_eq!(result, (2620 as u32 * 732923) >> 20);
}

