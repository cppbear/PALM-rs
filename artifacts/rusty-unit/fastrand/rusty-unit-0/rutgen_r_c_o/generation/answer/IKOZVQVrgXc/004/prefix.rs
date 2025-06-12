// Answer 0

#[test]
#[should_panic]
fn test_digit_base_zero() {
    let mut rng = Rng::with_seed(42);
    rng.digit(0);
}

#[test]
#[should_panic]
fn test_digit_base_greater_than_36() {
    let mut rng = Rng::with_seed(42);
    rng.digit(37);
}

#[test]
fn test_digit_base_36() {
    let mut rng = Rng::with_seed(42);
    let result = rng.digit(36);
}

#[test]
fn test_digit_base_10_boundary() {
    let mut rng = Rng::with_seed(42);
    let result = rng.digit(10);
}

