// Answer 0

#[test]
fn test_random_bool_valid_probability() {
    let true_count = (0..10000).filter(|_| rand::random_bool(0.75)).count();
    let false_count = 10000 - true_count;
    assert!(true_count > false_count); // expect true to be more likely
}

#[test]
#[should_panic]
fn test_random_bool_probability_too_high() {
    rand::random_bool(1.5);
}

#[test]
#[should_panic]
fn test_random_bool_probability_too_low() {
    rand::random_bool(-0.1);
}

#[test]
fn test_random_bool_exact_zero() {
    assert!(!rand::random_bool(0.0)); // expect false
}

#[test]
fn test_random_bool_exact_one() {
    assert!(rand::random_bool(1.0)); // expect true
}

