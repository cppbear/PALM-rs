// Answer 0

#[test]
#[should_panic(expected = "base cannot be zero")]
fn test_digit_base_zero() {
    let mut rng = fastrand::Rng::new(); // Assuming there's a way to initialize Rng
    rng.digit(0);
}

#[test]
#[should_panic(expected = "base cannot be larger than 36")]
fn test_digit_base_greater_than_36() {
    let mut rng = fastrand::Rng::new(); // Assuming there's a way to initialize Rng
    rng.digit(37);
}

#[test]
fn test_digit_base_36() {
    let mut rng = fastrand::Rng::new(); // Assuming there's a way to initialize Rng
    let result = rng.digit(36);
    assert!(result >= 'a' && result <= 'z', "Expected result to be between 'a' and 'z'");
}

#[test]
fn test_digit_at_boundary_value() {
    let mut rng = fastrand::Rng::new(); // Assuming there's a way to initialize Rng
    let result = rng.digit(36);
    assert_eq!(result, 'a' + 10 - 10, "Expected result to be 'a'");
}

