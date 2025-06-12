// Answer 0

#[test]
fn test_get_seed_zero() {
    let rng = Rng::with_seed(0);
    rng.get_seed();
}

#[test]
fn test_get_seed_one() {
    let rng = Rng::with_seed(1);
    rng.get_seed();
}

#[test]
fn test_get_seed_max() {
    let rng = Rng::with_seed(u64::MAX);
    rng.get_seed();
}

#[test]
fn test_get_seed_mid() {
    let rng = Rng::with_seed(u64::MAX / 2);
    rng.get_seed();
}

#[test]
fn test_get_seed_random() {
    let rng = Rng::with_seed(9876543210);
    rng.get_seed();
}

