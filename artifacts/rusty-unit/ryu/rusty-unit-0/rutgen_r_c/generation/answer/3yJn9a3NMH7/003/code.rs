// Answer 0

#[test]
#[should_panic(expected = "assertion failed: e >= 0")]
fn test_log2_pow5_negative_input() {
    let e = -1;
    log2_pow5(e);
}

#[test]
fn test_log2_pow5_zero_input() {
    let e = 0;
    assert_eq!(log2_pow5(e), 0);
}

#[test]
fn test_log2_pow5_maximum_valid_input() {
    let e = 3528;
    assert_eq!(log2_pow5(e), 4594260);
}

#[test]
#[should_panic(expected = "assertion failed: e <= 3528")]
fn test_log2_pow5_over_maximum_input() {
    let e = 3529;
    log2_pow5(e);
}

