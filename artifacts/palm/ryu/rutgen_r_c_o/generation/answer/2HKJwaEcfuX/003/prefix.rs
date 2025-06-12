// Answer 0

#[test]
#[should_panic]
fn test_log10_pow2_negative_value() {
    let e = -1;
    log10_pow2(e);
}

