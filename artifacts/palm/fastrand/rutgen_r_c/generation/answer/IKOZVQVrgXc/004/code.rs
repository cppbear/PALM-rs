// Answer 0

#[test]
#[should_panic(expected = "base cannot be zero")]
fn test_digit_zero_base() {
    let mut rng = Rng::with_seed(0);
    rng.digit(0);
}

#[test]
#[should_panic(expected = "base cannot be larger than 36")]
fn test_digit_base_greater_than_36() {
    let mut rng = Rng::with_seed(0);
    rng.digit(37);
}

#[test]
fn test_digit_base_boundary_case_36() {
    let mut rng = Rng::with_seed(0);
    let result = rng.digit(36);
    assert!(result >= 'a' && result <= 'z');
}

#[test]
fn test_digit_num_boundary_case_10() {
    let mut rng = Rng::with_seed(0);
    let result = rng.digit(10);
    assert!(result >= '0' && result <= '9');
}

