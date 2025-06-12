// Answer 0

#[test]
fn test_seed_initializes_rng() {
    struct Rng {
        seed_value: u64,
    }

    impl Rng {
        fn seed(&mut self, seed: u64) {
            self.seed_value = seed;
        }
    }

    fn with_rng<F>(f: F)
    where
        F: FnOnce(&mut Rng),
    {
        let mut rng = Rng { seed_value: 0 };
        f(&mut rng);
    }

    let initial_seed = 42;
    seed(initial_seed);

    let mut rng = Rng { seed_value: 0 };
    with_rng(|r| {
        r.seed(initial_seed);
    });

    assert_eq!(rng.seed_value, initial_seed);
}

#[test]
fn test_seed_with_different_values() {
    struct Rng {
        seed_value: u64,
    }

    impl Rng {
        fn seed(&mut self, seed: u64) {
            self.seed_value = seed;
        }
    }

    fn with_rng<F>(f: F)
    where
        F: FnOnce(&mut Rng),
    {
        let mut rng = Rng { seed_value: 0 };
        f(&mut rng);
    }

    let seed1 = 100;
    let seed2 = 200;

    seed(seed1);
    let mut rng1 = Rng { seed_value: 0 };
    with_rng(|r| {
        r.seed(seed1);
    });

    assert_eq!(rng1.seed_value, seed1);

    seed(seed2);
    let mut rng2 = Rng { seed_value: 0 };
    with_rng(|r| {
        r.seed(seed2);
    });

    assert_eq!(rng2.seed_value, seed2);
}

#[test]
#[should_panic]
fn test_seed_panic_on_zero_seed() {
    struct Rng {
        seed_value: u64,
    }

    impl Rng {
        fn seed(&mut self, seed: u64) {
            if seed == 0 {
                panic!("Seed cannot be zero");
            }
            self.seed_value = seed;
        }
    }

    fn with_rng<F>(f: F)
    where
        F: FnOnce(&mut Rng),
    {
        let mut rng = Rng { seed_value: 0 };
        f(&mut rng);
    }

    let zero_seed = 0;
    seed(zero_seed);
}

