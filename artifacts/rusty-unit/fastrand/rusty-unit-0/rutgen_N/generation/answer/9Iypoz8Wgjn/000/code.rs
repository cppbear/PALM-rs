// Answer 0

#[test]
fn test_alphabetic_generates_lowercase() {
    let mut rng = fastrand::Rng::new();
    let result = rng.alphabetic();
    assert!(result.is_ascii_lowercase());
}

#[test]
fn test_alphabetic_generates_uppercase() {
    let mut rng = fastrand::Rng::new();
    let result = rng.alphabetic();
    assert!(result.is_ascii_uppercase());
}

#[test]
fn test_alphabetic_generate_between_bounds() {
    let mut rng = fastrand::Rng::new();
    let result = rng.alphabetic();
    assert!(result >= 'A' && result <= 'Z' || result >= 'a' && result <= 'z');
}

#[test]
fn test_alphabetic_repeated_calls() {
    let mut rng = fastrand::Rng::new();
    let chars: Vec<char> = (0..100).map(|_| rng.alphabetic()).collect();
    for &c in &chars {
        assert!(c.is_ascii_alphabetic());
    }
}

