// Answer 0

#[test]
fn test_random_range_float_inclusive() {
    let result: f32 = rand::random_range(0.0..=1e9);
    assert!(result >= 0.0 && result <= 1e9);
}

#[test]
fn test_random_range_float_exclusive() {
    let result: f32 = rand::random_range(1e-5..1e5);
    assert!(result > 1e-5 && result < 1e5);
}

#[test]
fn test_random_range_integer() {
    let result: usize = rand::random_range(0..=100);
    assert!(result <= 100);
}

#[test]
fn test_random_range_empty_vector() {
    let words: Vec<&str> = Vec::new();
    let panic = std::panic::catch_unwind(|| {
        let _: usize = rand::random_range(..words.len());
    });
    assert!(panic.is_err());
}

#[test]
fn test_random_range_single_element() {
    let words: Vec<&str> = vec!["one"];
    let result: usize = rand::random_range(..words.len());
    assert_eq!(result, 0);
}

