// Answer 0

#[test]
fn test_random_range_float() {
    let result: f32 = rand::random_range(0.0..=1e9);
    assert!(result >= 0.0 && result <= 1e9);
}

#[test]
fn test_random_range_integer() {
    let result: usize = rand::random_range(0..=100);
    assert!(result <= 100);
}

#[test]
fn test_random_range_empty() {
    let result: usize = rand::random_range(..0);
    assert_eq!(result, 0); // Assuming it defaults to the start of the range
}

#[should_panic]
fn test_random_range_invalid() {
    let _result: usize = rand::random_range(100..0); // Invalid range should panic
}

