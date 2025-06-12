// Answer 0

#[test]
fn test_digit_zero_base() {
    let mut rng = Rng::with_seed(123);
    let result = std::panic::catch_unwind(|| {
        rng.digit(0);
    });
    assert!(result.is_err());
}

#[test]
fn test_digit_above_max_base() {
    let mut rng = Rng::with_seed(123);
    let result = std::panic::catch_unwind(|| {
        rng.digit(37);
    });
    assert!(result.is_err());
}

#[test]
fn test_digit_valid_base() {
    let mut rng = Rng::with_seed(123);
    let digit_char = rng.digit(10);
    assert!(digit_char >= '0' && digit_char <= '9');

    let digit_char = rng.digit(36);
    assert!(digit_char >= '0' && digit_char <= '9' || digit_char >= 'a' && digit_char <= 'z');
}

#[test]
fn test_digit_mid_range() {
    let mut rng = Rng::with_seed(456);
    let digit_char = rng.digit(36);
    assert!((digit_char.is_digit(10) || digit_char.is_ascii_lowercase()) && digit_char < 'a' + 10);
}

