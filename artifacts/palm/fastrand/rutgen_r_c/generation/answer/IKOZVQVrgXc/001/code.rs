// Answer 0

#[test]
fn test_digit_base_zero() {
    let mut rng = Rng::with_seed(12345);
    let result = std::panic::catch_unwind(|| {
        rng.digit(0);
    });
    assert!(result.is_err());
}

#[test]
fn test_digit_base_above_limit() {
    let mut rng = Rng::with_seed(12345);
    let result = std::panic::catch_unwind(|| {
        rng.digit(37);
    });
    assert!(result.is_err());
}

#[test]
fn test_digit_base_one() {
    let mut rng = Rng::with_seed(12345);
    let result = rng.digit(1);
    assert_eq!(result, '0'); // The only possible digit in base 1 would be 0
}

#[test]
fn test_digit_base_ten() {
    let mut rng = Rng::with_seed(12345);
    let result = rng.digit(10);
    assert!(result >= '0' && result <= '9');
}

#[test]
fn test_digit_base_thirty_six() {
    let mut rng = Rng::with_seed(12345);
    let result = rng.digit(36);
    assert!(result >= '0' && result <= '9' || result >= 'a' && result <= 'z');
}

#[test]
fn test_digit_edge_cases() {
    let mut rng = Rng::with_seed(12345);
    
    // Test base 2
    let result_base_2 = rng.digit(2);
    assert!(result_base_2 == '0' || result_base_2 == '1');

    // Test base 36, high range
    let result_base_36 = rng.digit(36);
    assert!(result_base_36 >= '0' && result_base_36 <= '9' || result_base_36 >= 'a' && result_base_36 <= 'z');
}

