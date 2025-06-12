// Answer 0

#[derive(Debug)]
struct Generator(u64);

impl Generator {
    pub fn new() -> Self {
        Generator(0)
    }

    pub fn seed(&mut self, seed: u64) {
        self.0 = seed;
    }

    pub fn get_seed(&self) -> u64 {
        self.0
    }
}

#[test]
fn test_seed_initializes_generator() {
    let mut gen = Generator::new();
    gen.seed(42);
    assert_eq!(gen.get_seed(), 42);
}

#[test]
fn test_seed_changes_existing_seed() {
    let mut gen = Generator::new();
    gen.seed(100);
    assert_eq!(gen.get_seed(), 100);
    gen.seed(250);
    assert_eq!(gen.get_seed(), 250);
}

#[test]
fn test_seed_zero_value() {
    let mut gen = Generator::new();
    gen.seed(0);
    assert_eq!(gen.get_seed(), 0);
}

#[test]
fn test_seed_large_value() {
    let mut gen = Generator::new();
    gen.seed(u64::MAX);
    assert_eq!(gen.get_seed(), u64::MAX);
}

