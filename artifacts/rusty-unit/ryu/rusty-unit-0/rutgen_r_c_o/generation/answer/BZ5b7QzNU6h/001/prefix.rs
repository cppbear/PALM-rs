// Answer 0

#[test]
fn test_ceil_log2_pow5_min_value() {
    let e = 0;
    ceil_log2_pow5(e);
}

#[test]
fn test_ceil_log2_pow5_small_value() {
    let e = 1;
    ceil_log2_pow5(e);
}

#[test]
fn test_ceil_log2_pow5_medium_value() {
    let e = 1000;
    ceil_log2_pow5(e);
}

#[test]
fn test_ceil_log2_pow5_large_value() {
    let e = 2000;
    ceil_log2_pow5(e);
}

#[test]
fn test_ceil_log2_pow5_max_value() {
    let e = 3528;
    ceil_log2_pow5(e);
}

#[test]
fn test_ceil_log2_pow5_boundary_value() {
    let e = 3527;
    ceil_log2_pow5(e);
}

