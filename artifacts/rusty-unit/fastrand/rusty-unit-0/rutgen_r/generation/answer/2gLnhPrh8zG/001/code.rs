// Answer 0

#[derive(Debug)]
struct SeedGenerator(u64);

impl SeedGenerator {
    pub fn get_seed(&self) -> u64 {
        self.0
    }
}

#[test]
fn test_get_seed_zero() {
    let generator = SeedGenerator(0);
    assert_eq!(generator.get_seed(), 0);
}

#[test]
fn test_get_seed_positive() {
    let generator = SeedGenerator(12345);
    assert_eq!(generator.get_seed(), 12345);
}

#[test]
fn test_get_seed_large() {
    let generator = SeedGenerator(u64::MAX);
    assert_eq!(generator.get_seed(), u64::MAX);
}

#[test]
fn test_get_seed_random() {
    let seed = 9876543210;
    let generator = SeedGenerator(seed);
    assert_eq!(generator.get_seed(), seed);
}

