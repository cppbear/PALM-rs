// Answer 0

#[test]
fn test_sample_true_case() {
    struct TestRng {
        value: u64,
    }

    impl Rng for TestRng {
        fn random(&mut self) -> u64 {
            self.value
        }
    }

    let mut rng = TestRng { value: 1 };
    let instance = Bernoulli { p_int: 1 }; // Assuming ALWAYS_TRUE corresponds to 1
    assert_eq!(instance.sample(&mut rng), true);
}

#[test]
fn test_sample_false_case() {
    struct TestRng {
        value: u64,
    }

    impl Rng for TestRng {
        fn random(&mut self) -> u64 {
            self.value
        }
    }

    let mut rng = TestRng { value: 0 };
    let instance = Bernoulli { p_int: 1 }; // Assuming NEVER_TRUE corresponds to values greater than 0
    assert_eq!(instance.sample(&mut rng), false);
}

#[test]
fn test_sample_boundary_case() {
    struct TestRng {
        value: u64,
    }

    impl Rng for TestRng {
        fn random(&mut self) -> u64 {
            self.value
        }
    }

    let mut rng_min_true = TestRng { value: 0 };
    let instance_min = Bernoulli { p_int: 1 };
    assert_eq!(instance_min.sample(&mut rng_min_true), true);
    
    let mut rng_max_false = TestRng { value: 1 };
    let instance_max = Bernoulli { p_int: 1 };
    assert_eq!(instance_max.sample(&mut rng_max_false), true);
}

