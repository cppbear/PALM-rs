// Answer 0

#[test]
#[should_panic]
fn test_log10_pow2_e_greater_than_1650() {
    let e = 1651;
    log10_pow2(e);
}

#[test]
fn test_log10_pow2_e_is_0() {
    let e = 0;
    log10_pow2(e);
}

#[test]
fn test_log10_pow2_e_is_1() {
    let e = 1;
    log10_pow2(e);
}

#[test]
fn test_log10_pow2_e_is_1650() {
    let e = 1650;
    log10_pow2(e);
}

#[test]
fn test_log10_pow2_e_is_1000() {
    let e = 1000;
    log10_pow2(e);
}

