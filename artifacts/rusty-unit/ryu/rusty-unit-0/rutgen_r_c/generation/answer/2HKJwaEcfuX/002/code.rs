// Answer 0

#[test]
fn test_log10_pow2_lower_bound() {
    let result = log10_pow2(0);
    assert_eq!(result, 0);
}

#[test]
#[should_panic]
fn test_log10_pow2_upper_bound_exceed() {
    let _result = log10_pow2(1651);
}

