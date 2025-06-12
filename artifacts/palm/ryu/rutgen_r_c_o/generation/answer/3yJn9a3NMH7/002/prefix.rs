// Answer 0

#[test]
fn test_log2_pow5_lower_bound() {
    let result = log2_pow5(0);
}

#[test]
fn test_log2_pow5_mid_range() {
    let result = log2_pow5(1764);
}

#[test]
fn test_log2_pow5_upper_bound() {
    let result = log2_pow5(3528);
}

#[should_panic]
fn test_log2_pow5_beyond_upper_bound() {
    let result = log2_pow5(3529);
}

