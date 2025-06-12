// Answer 0

#[test]
fn test_gen_mod_u64_lower_bound() {
    struct RandomGenerator {
        value: u64,
    }

    impl RandomGenerator {
        fn gen_u64(&mut self) -> u64 {
            self.value
        }
    }

    let mut rng = RandomGenerator { value: 1 };
    let n = 1;
    let result = rng.gen_mod_u64(n);
    assert_eq!(result, 0);
}

#[test]
fn test_gen_mod_u64_mid_range() {
    struct RandomGenerator {
        value: u64,
    }

    impl RandomGenerator {
        fn gen_u64(&mut self) -> u64 {
            self.value
        }
    }

    let mut rng = RandomGenerator { value: 2 };
    let n = 5; // arbitrary n > lo
    let result = rng.gen_mod_u64(n);
    assert_eq!(result, 0); // hi calculation should output valid value
}

#[test]
fn test_gen_mod_u64_high_value() {
    struct RandomGenerator {
        value: u64,
    }

    impl RandomGenerator {
        fn gen_u64(&mut self) -> u64 {
            self.value
        }
    }

    let mut rng = RandomGenerator { value: u64::MAX }; // max u64 value
    let n = u64::MAX; // testing max boundary
    let result = rng.gen_mod_u64(n);
    assert_eq!(result, 0); // expects 0 since lo < n is false
}

#[test]
#[should_panic]
fn test_gen_mod_u64_zero() {
    struct RandomGenerator {
        value: u64,
    }

    impl RandomGenerator {
        fn gen_u64(&mut self) -> u64 {
            self.value
        }
    }

    let mut rng = RandomGenerator { value: 1 };
    let n = 0; // should panic as n == 0 is not a valid case for the function
    rng.gen_mod_u64(n);
}

