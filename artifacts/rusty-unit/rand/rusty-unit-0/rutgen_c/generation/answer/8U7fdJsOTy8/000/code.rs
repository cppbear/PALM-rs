// Answer 0

#[test]
fn test_sample_always_true() {
    struct MockRng {
        return_value: u64,
    }

    impl Rng for MockRng {
        fn random(&mut self) -> u64 {
            self.return_value
        }
    }

    let rng = &mut MockRng { return_value: 0 };
    let bernoulli = Bernoulli { p_int: ALWAYS_TRUE };
    assert_eq!(bernoulli.sample(rng), true);
}

#[test]
fn test_sample_success() {
    struct MockRng {
        return_value: u64,
    }

    impl Rng for MockRng {
        fn random(&mut self) -> u64 {
            self.return_value
        }
    }

    let rng = &mut MockRng { return_value: 5 };
    let bernoulli = Bernoulli { p_int: 10 };
    assert_eq!(bernoulli.sample(rng), true);
}

#[test]
fn test_sample_failure() {
    struct MockRng {
        return_value: u64,
    }

    impl Rng for MockRng {
        fn random(&mut self) -> u64 {
            self.return_value
        }
    }

    let rng = &mut MockRng { return_value: 15 };
    let bernoulli = Bernoulli { p_int: 10 };
    assert_eq!(bernoulli.sample(rng), false);
}

