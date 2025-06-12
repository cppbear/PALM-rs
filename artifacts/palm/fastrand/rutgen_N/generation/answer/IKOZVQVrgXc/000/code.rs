// Answer 0

#[test]
fn test_digit_base_1() {
    let mut rng = fastrand::Rng::default();
    let result = rng.digit(1);
    assert_eq!(result, '0');
}

#[test]
fn test_digit_base_10() {
    let mut rng = fastrand::Rng::default();
    let result = rng.digit(10);
    assert!(result.is_ascii_digit());
}

#[test]
fn test_digit_base_36() {
    let mut rng = fastrand::Rng::default();
    let result = rng.digit(36);
    assert!(result.is_ascii_alphanumeric());
}

#[should_panic(expected = "base cannot be zero")]
#[test]
fn test_digit_base_zero() {
    let mut rng = fastrand::Rng::default();
    rng.digit(0);
}

#[should_panic(expected = "base cannot be larger than 36")]
#[test]
fn test_digit_base_large() {
    let mut rng = fastrand::Rng::default();
    rng.digit(37);
}

