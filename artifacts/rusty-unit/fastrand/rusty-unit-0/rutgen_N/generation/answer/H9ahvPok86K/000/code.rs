// Answer 0

#[derive(Debug)]
struct Rng(u64);

impl Rng {
    pub fn with_seed(seed: u64) -> Self {
        Rng(seed)
    }
}

#[test]
fn test_rng_with_seed() {
    let seed = 12345;
    let rng = Rng::with_seed(seed);
    assert_eq!(rng.0, seed);
}

#[test]
fn test_rng_with_zero_seed() {
    let seed = 0;
    let rng = Rng::with_seed(seed);
    assert_eq!(rng.0, seed);
}

#[test]
fn test_rng_with_large_seed() {
    let seed = u64::MAX;
    let rng = Rng::with_seed(seed);
    assert_eq!(rng.0, seed);
}

