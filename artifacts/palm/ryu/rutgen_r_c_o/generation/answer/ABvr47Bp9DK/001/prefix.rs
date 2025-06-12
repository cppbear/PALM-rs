// Answer 0

#[test]
fn test_log10_pow5_zero() {
    let e = 0;
    let result = log10_pow5(e);
}

#[test]
fn test_log10_pow5_max() {
    let e = 2620;
    let result = log10_pow5(e);
}

#[test]
fn test_log10_pow5_mid() {
    let e = 1310;
    let result = log10_pow5(e);
}

#[test]
fn test_log10_pow5_small() {
    let e = 5;
    let result = log10_pow5(e);
}

#[test]
fn test_log10_pow5_large() {
    let e = 2000;
    let result = log10_pow5(e);
}

