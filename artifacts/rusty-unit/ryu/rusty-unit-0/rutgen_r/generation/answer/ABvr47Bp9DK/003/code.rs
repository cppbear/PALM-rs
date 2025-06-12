// Answer 0

#[test]
#[should_panic]
fn test_log10_pow5_negative_e() {
    let _result = log10_pow5(-1);
}

#[test]
fn test_log10_pow5_zero() {
    let result = log10_pow5(0);
    assert_eq!(result, 0);
}

#[test]
fn test_log10_pow5_boundary_min() {
    let result = log10_pow5(1);
    assert_eq!(result, 0);
}

#[test]
fn test_log10_pow5_boundary_max() {
    let result = log10_pow5(2620);
    assert_eq!(result, 732923);
}

#[test]
fn test_log10_pow5_within_bounds() {
    let result = log10_pow5(1000);
    assert_eq!(result, 351562);
}

