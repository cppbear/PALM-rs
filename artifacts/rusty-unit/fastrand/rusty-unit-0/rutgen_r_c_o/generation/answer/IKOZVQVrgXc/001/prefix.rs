// Answer 0

#[test]
fn test_digit_base_1() {
    let mut rng = Rng::with_seed(1);
    let result = rng.digit(1);
}

#[test]
fn test_digit_base_2() {
    let mut rng = Rng::with_seed(2);
    let result = rng.digit(2);
}

#[test]
fn test_digit_base_10() {
    let mut rng = Rng::with_seed(3);
    let result = rng.digit(10);
}

#[test]
fn test_digit_base_11() {
    let mut rng = Rng::with_seed(4);
    let result = rng.digit(11);
}

#[test]
fn test_digit_base_36() {
    let mut rng = Rng::with_seed(5);
    let result = rng.digit(36);
}

#[should_panic(expected = "base cannot be zero")]
#[test]
fn test_digit_base_0() {
    let mut rng = Rng::with_seed(6);
    let result = rng.digit(0);
}

#[should_panic(expected = "base cannot be larger than 36")]
#[test]
fn test_digit_base_37() {
    let mut rng = Rng::with_seed(7);
    let result = rng.digit(37);
}

