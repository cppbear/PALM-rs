// Answer 0

#[cfg(test)]
fn test_sample() {
    struct TestRng {
        value: u32,
    }

    impl Rng for TestRng {
        fn random<T>(&mut self) -> T {
            // In a real RNG, we'd need proper random behavior,
            // but we can ensure valid char ranges for testing.
            if self.value < 0xD800 {
                self.value += 1; // Increment to simulate randomness
                return unsafe { std::mem::transmute(self.value as u32) };
            } else if self.value >= 0xD800 && self.value < 0x110000 {
                self.value += 1; // Skip surrogate range
                return unsafe { std::mem::transmute(self.value as u32) };
            }
            self.value = 0; // Reset for repeatability
            unsafe { std::mem::transmute(self.value as u32) }
        }
    }

    let mut rng = TestRng { value: 0 };

    let standard_uniform = StandardUniform;

    for _ in 0..10 {
        let result = standard_uniform.sample(&mut rng);
        assert!((result as u32) < 0xD800 || (result as u32) >= 0xDFFF);
    }
}

#[test]
fn test_sample_function() {
    test_sample();
}

