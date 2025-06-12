// Answer 0

#[test]
fn test_sample_always_true() {
    struct MockRng {
        value: u64,
    }

    impl Rng for MockRng {
        fn random(&mut self) -> u64 {
            self.value
        }
    }

    let rng = &mut MockRng { value: 0 };
    let bernoulli = Bernoulli { p_int: ALWAYS_TRUE };
    assert_eq!(bernoulli.sample(rng), true);
}

