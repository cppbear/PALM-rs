// Answer 0

#[test]
fn test_with_seed_min() {
    let rng = Rng::with_seed(0);
}

#[test]
fn test_with_seed_max() {
    let rng = Rng::with_seed(18446744073709551615);
}

#[test]
fn test_with_seed_mid() {
    let rng = Rng::with_seed(9223372036854775807);
}

#[test]
fn test_with_seed_random() {
    let rng = Rng::with_seed(123456789);
}

#[test]
fn test_with_seed_large() {
    let rng = Rng::with_seed(9876543210987654321);
}

