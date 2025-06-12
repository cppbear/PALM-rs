// Answer 0

#[derive(Debug)]
struct Rng {
    seed: u64,
}

impl Rng {
    fn from_seed(seed: u64) -> Self {
        Rng { seed }
    }
}

struct StdRng(Rng);

impl StdRng {
    fn from_seed(seed: u64) -> Self {
        StdRng(Rng::from_seed(seed))
    }
}

#[test]
fn test_std_rng_from_seed_zero() {
    let seed = 0;
    let rng = StdRng::from_seed(seed);
    assert_eq!(rng.0.seed, seed);
}

#[test]
fn test_std_rng_from_seed_max() {
    let seed = u64::MAX;
    let rng = StdRng::from_seed(seed);
    assert_eq!(rng.0.seed, seed);
}

#[test]
fn test_std_rng_from_seed_one() {
    let seed = 1;
    let rng = StdRng::from_seed(seed);
    assert_eq!(rng.0.seed, seed);
}

#[test]
fn test_std_rng_from_seed_large_value() {
    let seed = 12345678901234567890;
    let rng = StdRng::from_seed(seed);
    assert_eq!(rng.0.seed, seed);
}

