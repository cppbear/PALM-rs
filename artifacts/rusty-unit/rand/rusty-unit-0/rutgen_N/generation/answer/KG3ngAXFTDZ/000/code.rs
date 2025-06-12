// Answer 0

#[test]
fn test_random_bool_zero_probability() {
    let result = rand::random_bool(0.0);
    assert!(!result);
}

#[test]
fn test_random_bool_full_probability() {
    let result = rand::random_bool(1.0);
    assert!(result);
}

#[test]
fn test_random_bool_half_probability() {
    let true_count = (0..1000).filter(|_| rand::random_bool(0.5)).count();
    assert!(true_count > 300 && true_count < 700);
}

#[should_panic]
#[test]
fn test_random_bool_negative_probability() {
    rand::random_bool(-0.1);
}

#[should_panic]
#[test]
fn test_random_bool_above_one_probability() {
    rand::random_bool(1.1);
}

