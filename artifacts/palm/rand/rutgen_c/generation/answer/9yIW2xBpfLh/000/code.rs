// Answer 0

#[test]
fn test_random_range_floats() {
    let result: f32 = rand::random_range(0.0..=1e9);
    assert!(result >= 0.0 && result <= 1e9);
}

#[test]
fn test_random_range_integers() {
    let result: i32 = rand::random_range(1..=100);
    assert!(result >= 1 && result <= 100);
}

#[test]
fn test_random_range_empty() {
    let words: Vec<&str> = Vec::new();
    let result: usize = rand::random_range(..words.len());
    assert!(result == 0); // Should panic or handle when empty
}

#[should_panic(expected = "panicked at 'index out of bounds'")]
#[test]
fn test_random_range_out_of_bounds() {
    let words: Vec<&str> = "Hello World".split(' ').collect();
    let result: usize = rand::random_range(..words.len());
    assert!(result < words.len());
}

