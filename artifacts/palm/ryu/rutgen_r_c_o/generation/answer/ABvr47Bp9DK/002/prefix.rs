// Answer 0

#[test]
fn test_log10_pow5_zero() {
    let e = 0;
    log10_pow5(e);
}

#[test]
fn test_log10_pow5_min_value() {
    let e = 1;
    log10_pow5(e);
}

#[test]
fn test_log10_pow5_middle_value() {
    let e = 1310;
    log10_pow5(e);
}

#[test]
fn test_log10_pow5_max_value() {
    let e = 2620;
    log10_pow5(e);
}

#[should_panic]
fn test_log10_pow5_too_large() {
    let e = 2621;
    log10_pow5(e);
}

