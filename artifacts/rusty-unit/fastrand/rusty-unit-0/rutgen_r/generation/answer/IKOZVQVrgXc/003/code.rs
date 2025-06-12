// Answer 0

#[test]
#[should_panic(expected = "base cannot be zero")]
fn test_digit_base_zero() {
    let mut rng = fastrand::Rng::new();
    rng.digit(0);
}

#[test]
fn test_digit_base_max() {
    let mut rng = fastrand::Rng::new();
    for _ in 0..100 {
        let result = rng.digit(36);
        assert!(result.is_ascii_alphanumeric());
        assert!(result >= 'a' && result <= 'z');
    }
}

#[test]
fn test_digit_base_boundary() {
    let mut rng = fastrand::Rng::new();
    for _ in 0..100 {
        let result = rng.digit(10);
        assert!(result.is_ascii_digit());
        assert!(result >= '0' && result <= '9');
    }
}

#[test]
#[should_panic(expected = "base cannot be larger than 36")]
fn test_digit_base_above_max() {
    let mut rng = fastrand::Rng::new();
    rng.digit(37);
}

