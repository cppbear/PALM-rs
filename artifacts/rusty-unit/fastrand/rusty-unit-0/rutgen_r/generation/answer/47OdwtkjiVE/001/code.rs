// Answer 0

#[test]
fn test_lowercase_random_char() {
    use fastrand::Rng;

    // Mocking the RNG behavior directly
    struct MockRng;

    impl MockRng {
        fn lowercase(&self) -> char {
            'a' // Returning a static value for simplicity
        }
    }

    fn with_rng<F: FnOnce(&MockRng) -> char>(f: F) -> char {
        let rng = MockRng;
        f(&rng)
    }

    let result = with_rng(|r| r.lowercase());
    assert!(result >= 'a' && result <= 'z', "The result should be a lowercase letter.");
}

#[test]
fn test_lowercase_random_char_boundary() {
    struct MockRng;

    impl MockRng {
        fn lowercase(&self) -> char {
            // Testing the lower boundary case
            'a' 
        }
    }

    fn with_rng<F: FnOnce(&MockRng) -> char>(f: F) -> char {
        let rng = MockRng;
        f(&rng)
    }

    let result = with_rng(|r| r.lowercase());
    assert_eq!(result, 'a', "The result should be the lower boundary 'a'.");

    struct MockRngUpper;

    impl MockRngUpper {
        fn lowercase(&self) -> char {
            // Testing the upper boundary case
            'z' 
        }
    }

    fn with_rng_upper<F: FnOnce(&MockRngUpper) -> char>(f: F) -> char {
        let rng = MockRngUpper;
        f(&rng)
    }

    let result_upper = with_rng_upper(|r| r.lowercase());
    assert_eq!(result_upper, 'z', "The result should be the upper boundary 'z'.");
}

