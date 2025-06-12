// Answer 0

#[test]
fn test_lowercase_function() {
    let mut rng = Rng::with_seed(1);
    let result = rng.lowercase();
    assert!(result.is_ascii_lowercase());
    assert!(result >= 'a' && result <= 'z');
}

#[test]
fn test_lowercase_function_with_different_seed() {
    let mut rng = Rng::with_seed(2);
    let result = rng.lowercase();
    assert!(result.is_ascii_lowercase());
    assert!(result >= 'a' && result <= 'z');
}

#[test]
fn test_lowercase_function_multiple_calls() {
    let mut rng = Rng::with_seed(3);
    let results: Vec<char> = (0..10).map(|_| rng.lowercase()).collect();
    for result in results {
        assert!(result.is_ascii_lowercase());
        assert!(result >= 'a' && result <= 'z');
    }
}

