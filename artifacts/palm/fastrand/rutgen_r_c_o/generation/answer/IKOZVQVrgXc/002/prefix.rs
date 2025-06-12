// Answer 0

#[test]
#[should_panic]
fn test_digit_base_zero() {
    let mut rng = Rng::with_seed(123);
    rng.digit(0);
}

#[test]
#[should_panic]
fn test_digit_base_greater_than_36() {
    let mut rng = Rng::with_seed(123);
    rng.digit(37);
}

