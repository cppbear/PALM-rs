// Answer 0

#[test]
fn test_sample_always_true() {
    struct MockRng;

    impl Rng for MockRng {
        fn random(&mut self) -> u64 {
            0 // Dummy implementation
        }
    }

    let bernoulli = Bernoulli {
        p_int: u64::MAX,
    };
    let mut rng = MockRng;

    let result = bernoulli.sample(&mut rng);
}

#[test]
fn test_sample_always_true_with_different_rng() {
    struct AnotherMockRng;

    impl Rng for AnotherMockRng {
        fn random(&mut self) -> u64 {
            0 // Dummy implementation
        }
    }

    let bernoulli = Bernoulli {
        p_int: u64::MAX,
    };
    let mut rng = AnotherMockRng;

    let result = bernoulli.sample(&mut rng);
}

