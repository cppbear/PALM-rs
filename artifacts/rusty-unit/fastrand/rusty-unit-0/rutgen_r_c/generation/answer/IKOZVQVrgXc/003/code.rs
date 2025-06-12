// Answer 0

#[test]
#[should_panic(expected = "base cannot be zero")]
fn test_digit_with_zero_base() {
    let mut rng = Rng::with_seed(123);
    rng.digit(0);
}

#[test]
#[should_panic(expected = "base cannot be larger than 36")]
fn test_digit_with_large_base() {
    let mut rng = Rng::with_seed(123);
    rng.digit(37);
}

#[test]
fn test_digit_with_valid_base() {
    let mut rng = Rng::with_seed(123);
    let result = rng.digit(10);
    assert!(result >= '0' && result <= '9');
}

#[test]
fn test_digit_with_lowercase_base() {
    let mut rng = Rng::with_seed(123);
    let result = rng.digit(36);
    assert!(result >= 'a' && result <= 'z' || result >= '0' && result <= '9');
}

#[test]
fn test_digit_with_upper_limit_base() {
    let mut rng = Rng::with_seed(123);
    let result = rng.digit(10);
    assert!(result >= '0' && result <= '9');
}

