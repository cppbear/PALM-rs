// Answer 0

#[test]
#[should_panic(expected = "base cannot be zero")]
fn test_digit_zero_base() {
    let mut rng = fastrand::Rng::new();
    rng.digit(0);
}

#[test]
#[should_panic(expected = "base cannot be larger than 36")]
fn test_digit_large_base() {
    let mut rng = fastrand::Rng::new();
    rng.digit(37);
}

#[test]
fn test_digit_valid_base_10() {
    let mut rng = fastrand::Rng::new();
    let result = rng.digit(10);
    assert!(result >= '0' && result <= '9');
}

#[test]
fn test_digit_valid_base_36() {
    let mut rng = fastrand::Rng::new();
    let result = rng.digit(36);
    assert!((result >= '0' && result <= '9') || (result >= 'a' && result <= 'z'));
}

#[test]
fn test_digit_valid_base_16() {
    let mut rng = fastrand::Rng::new();
    let result = rng.digit(16);
    assert!(result >= '0' && result <= '9' || result >= 'a' && result <= 'f');
}

#[test]
fn test_digit_valid_base_2() {
    let mut rng = fastrand::Rng::new();
    let result = rng.digit(2);
    assert!(result == '0' || result == '1');
}

