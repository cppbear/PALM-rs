// Answer 0

#[test]
fn test_f64_in_range() {
    struct TestRng {
        seed: u64,
    }

    impl TestRng {
        fn u64(&mut self, range: std::ops::Range<usize>) -> u64 {
            self.seed = self.seed.wrapping_mul(6364136223846793005).wrapping_add(1);
            (self.seed >> range.start as u64) & ((1 << (range.end - range.start)) - 1)
        }

        fn f64(&mut self) -> f64 {
            let b = 64;
            let f = core::f64::MANTISSA_DIGITS - 1;
            f64::from_bits((1 << (b - 2)) - (1 << f) + (self.u64(..) >> (b - f))) - 1.0
        }
    }

    let mut rng = TestRng { seed: 42 };

    // Test the boundary conditions, especially around the expected output bounds
    let val1 = rng.f64();
    assert!(val1 >= 0.0 && val1 < 1.0, "Value out of range: {}", val1);
    
    let val2 = rng.f64();
    assert!(val2 >= 0.0 && val2 < 1.0, "Value out of range: {}", val2);
    
    let val3 = rng.f64();
    assert!(val3 >= 0.0 && val3 < 1.0, "Value out of range: {}", val3);
}

