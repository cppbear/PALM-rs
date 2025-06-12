// Answer 0

#[test]
#[should_panic(expected = "base cannot be zero")]
fn test_digit_with_base_zero() {
    let mut rng = Rng::with_seed(42);
    rng.digit(0);
}

#[test]
#[should_panic(expected = "base cannot be larger than 36")]
fn test_digit_with_base_greater_than_36() {
    let mut rng = Rng::with_seed(42);
    rng.digit(37);
}

