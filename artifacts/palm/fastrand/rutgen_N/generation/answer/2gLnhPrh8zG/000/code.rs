// Answer 0

#[test]
fn test_get_seed() {
    struct SeedGenerator(u64);

    impl SeedGenerator {
        pub fn new(seed: u64) -> Self {
            SeedGenerator(seed)
        }

        pub fn get_seed(&self) -> u64 {
            self.0
        }
    }

    let generator = SeedGenerator::new(42);
    assert_eq!(generator.get_seed(), 42);
}

#[test]
fn test_get_seed_with_different_value() {
    struct SeedGenerator(u64);

    impl SeedGenerator {
        pub fn new(seed: u64) -> Self {
            SeedGenerator(seed)
        }

        pub fn get_seed(&self) -> u64 {
            self.0
        }
    }

    let generator = SeedGenerator::new(100);
    assert_eq!(generator.get_seed(), 100);
}

#[test]
fn test_get_seed_zero() {
    struct SeedGenerator(u64);

    impl SeedGenerator {
        pub fn new(seed: u64) -> Self {
            SeedGenerator(seed)
        }

        pub fn get_seed(&self) -> u64 {
            self.0
        }
    }

    let generator = SeedGenerator::new(0);
    assert_eq!(generator.get_seed(), 0);
}

