// Answer 0

#[test]
fn test_gen_u128() {
    struct RandomGen;

    impl RandomGen {
        fn gen_u64(&mut self) -> u64 {
            42 // fixed value for consistent testing
        }

        fn gen_u128(&mut self) -> u128 {
            (u128::from(self.gen_u64()) << 64) | u128::from(self.gen_u64())
        }
    }

    let mut rng = RandomGen;
    let result = rng.gen_u128();
    assert_eq!(result, (42u128 << 64) | 42);
}

#[test]
fn test_gen_u128_boundary() {
    struct RandomGenBoundary;

    impl RandomGenBoundary {
        fn gen_u64(&mut self) -> u64 {
            u64::MAX // boundary value for testing
        }

        fn gen_u128(&mut self) -> u128 {
            (u128::from(self.gen_u64()) << 64) | u128::from(self.gen_u64())
        }
    }

    let mut rng = RandomGenBoundary;
    let result = rng.gen_u128();
    assert_eq!(result, (u128::MAX << 64) | u128::MAX);
}

