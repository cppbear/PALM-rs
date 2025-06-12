// Answer 0

#[test]
#[should_panic]
fn test_digit_base_zero() {
    let mut rng = Rng::with_seed(0);
    rng.digit(0);
}

#[test]
fn test_digit_base_max() {
    let mut rng = Rng::with_seed(1);
    let _ = rng.digit(36);
}

#[test]
fn test_digit_base_less_than_ten() {
    let mut rng = Rng::with_seed(2);
    let _ = rng.digit(10);
}

#[test]
fn test_digit_base_nine() {
    let mut rng = Rng::with_seed(3);
    let _ = rng.digit(9);
}

