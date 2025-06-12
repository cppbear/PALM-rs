// Answer 0

#[test]
fn test_f32_random_in_range() {
    struct TestRand {
        value: u32,
    }

    impl TestRand {
        fn u32(&mut self, _range: std::ops::Range<usize>) -> u32 {
            self.value
        }

        fn new(value: u32) -> Self {
            TestRand { value }
        }
    }

    let mut rng = TestRand::new(1073741824); // Example value for testing
    let result = fastrand::f32(&mut rng);
    assert!(result >= 0.0 && result < 1.0);
}

#[test]
fn test_f32_random_boundary() {
    struct TestRand {
        value: u32,
    }

    impl TestRand {
        fn u32(&mut self, _range: std::ops::Range<usize>) -> u32 {
            self.value
        }

        fn new(value: u32) -> Self {
            TestRand { value }
        }
    }

    // Test lower boundary
    let mut rng_low = TestRand::new(0);
    let result_low = fastrand::f32(&mut rng_low);
    assert_eq!(result_low, 0.0);

    // Test upper boundary
    let mut rng_high = TestRand::new(u32::MAX);
    let result_high = fastrand::f32(&mut rng_high);
    assert!(result_high < 1.0);
}

