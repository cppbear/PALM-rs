// Answer 0

#[test]
fn test_p_always_true() {
    struct DummyRng;

    impl Rng for DummyRng {
        // Implement necessary methods for Rng trait (if needed by surrounding context)
    }

    let bernoulli = Bernoulli { p_int: ALWAYS_TRUE };
    assert_eq!(bernoulli.p(), 1.0);
}

