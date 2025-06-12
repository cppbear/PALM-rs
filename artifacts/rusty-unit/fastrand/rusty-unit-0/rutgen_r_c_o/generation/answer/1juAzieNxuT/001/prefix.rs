// Answer 0

#[test]
fn test_lowercase_valid() {
    let mut rng = Rng::with_seed(1);
    let result = rng.lowercase();
}

#[test]
fn test_lowercase_another_seed() {
    let mut rng = Rng::with_seed(42);
    let result = rng.lowercase();
}

#[test]
fn test_lowercase_edge_case() {
    let mut rng = Rng::with_seed(0);
    let result = rng.lowercase();
}

#[test]
fn test_lowercase_random_seed() {
    let mut rng = Rng::with_seed(100);
    let result = rng.lowercase();
}

