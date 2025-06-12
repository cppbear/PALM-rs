// Answer 0

#[test]
fn test_bool_returns_boolean() {
    let result = fastrand::bool();
    assert!(result == true || result == false);
}

#[test]
fn test_bool_is_deterministic() {
    // Create a fixed RNG for deterministic output.
    struct FixedRng {
        value: bool,
    }

    impl FixedRng {
        fn new(value: bool) -> Self {
            Self { value }
        }

        fn bool(&self) -> bool {
            self.value
        }
    }

    fn with_rng<F: FnOnce(&FixedRng)>(f: F) {
        let rng = FixedRng::new(true); // Fixed to always return `true`.
        f(&rng);
    }

    let mut true_count = 0;
    for _ in 0..100 {
        with_rng(|r| {
            if r.bool() {
                true_count += 1;
            }
        });
    }
    assert_eq!(true_count, 100); // Should always be true
}

#[test]
fn test_bool_varies_with_rng() {
    struct VariedRng {
        counter: usize,
    }

    impl VariedRng {
        fn new() -> Self {
            Self { counter: 0 }
        }

        fn bool(&mut self) -> bool {
            self.counter += 1;
            self.counter % 2 == 0 // Alternates true/false
        }
    }

    fn with_rng<F: FnOnce(&mut VariedRng)>(f: F) {
        let mut rng = VariedRng::new();
        f(&mut rng);
    }

    let mut true_count = 0;
    let mut false_count = 0;
    for _ in 0..100 {
        with_rng(|r| {
            if r.bool() {
                true_count += 1;
            } else {
                false_count += 1;
            }
        });
    }
    
    assert!(true_count > 0);
    assert!(false_count > 0);
}

