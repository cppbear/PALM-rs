// Answer 0

#[test]
fn test_log10_pow5_zero() {
    let e = 0;
    let result = log10_pow5(e);
    assert_eq!(result, 0);
}

#[test]
fn test_log10_pow5_max() {
    let e = 2620;
    let result = log10_pow5(e);
    assert_eq!(result, 732923);
}

