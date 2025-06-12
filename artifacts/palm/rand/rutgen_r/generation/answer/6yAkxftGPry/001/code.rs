// Answer 0

#[derive(Debug)]
struct R;

impl R {
    fn seed_from_u64(seed: u64) -> u64 {
        seed ^ 0x5DEECE66D ^ ((seed << 16) | (seed >> 48))
    }
}

#[derive(Debug)]
struct Seeded {
    value: u64,
}

impl Seeded {
    fn new(value: u64) -> Self {
        Seeded { value }
    }

    fn seed_from_u64(seed: u64) -> Self {
        Self::new(R::seed_from_u64(seed))
    }
}

#[test]
fn test_seed_from_u64_zero() {
    let seed = 0;
    let seeded = Seeded::seed_from_u64(seed);
    assert_eq!(seeded.value, R::seed_from_u64(seed));
}

#[test]
fn test_seed_from_u64_max() {
    let seed = 0xFFFFFFFFFFFFFFFF;
    let seeded = Seeded::seed_from_u64(seed);
    assert_eq!(seeded.value, R::seed_from_u64(seed));
}

#[test]
fn test_seed_from_u64_small() {
    let seed = 1;
    let seeded = Seeded::seed_from_u64(seed);
    assert_eq!(seeded.value, R::seed_from_u64(seed));
}

#[test]
fn test_seed_from_u64_mid() {
    let seed = 123456789;
    let seeded = Seeded::seed_from_u64(seed);
    assert_eq!(seeded.value, R::seed_from_u64(seed));
}

