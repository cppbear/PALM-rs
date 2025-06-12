// Answer 0

#[test]
fn test_sample_below_probability() {
    struct MockRng {
        value: u64,
    }

    impl Rng for MockRng {
        fn random(&mut self) -> u64 {
            self.value
        }
    }

    let bernoulli = Bernoulli { p_int: 10 };
    let mut rng = MockRng { value: 5 };
    let result = bernoulli.sample(&mut rng);
    assert_eq!(result, true);
}

#[test]
fn test_sample_at_probability() {
    struct MockRng {
        value: u64,
    }

    impl Rng for MockRng {
        fn random(&mut self) -> u64 {
            self.value
        }
    }

    let bernoulli = Bernoulli { p_int: 10 };
    let mut rng = MockRng { value: 10 };
    let result = bernoulli.sample(&mut rng);
    assert_eq!(result, false);
}

#[test]
fn test_sample_above_probability() {
    struct MockRng {
        value: u64,
    }

    impl Rng for MockRng {
        fn random(&mut self) -> u64 {
            self.value
        }
    }

    let bernoulli = Bernoulli { p_int: 10 };
    let mut rng = MockRng { value: 15 };
    let result = bernoulli.sample(&mut rng);
    assert_eq!(result, false);
}

