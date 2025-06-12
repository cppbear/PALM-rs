// Answer 0

#[test]
fn test_reseeding_core_new_zero_threshold() {
    struct TestRng;

    impl SeedableRng for TestRng {
        type Seed = [u8; 32];

        fn from_seed(seed: Self::Seed) -> Self {
            TestRng
        }
    }

    impl TryRngCore for TestRng {
        type Error = ();
        fn try_from_rng<R: RngCore>(rng: &mut R) -> Result<Self, Self::Error> {
            Ok(TestRng)
        }
    }

    let reseeder = TestRng;
    let result = ReseedingCore::<TestRng, TestRng>::new(0, reseeder);
    assert!(result.is_ok());
    let core = result.unwrap();
    assert_eq!(core.threshold, i64::MAX);
}

#[test]
fn test_reseeding_core_new_valid_threshold() {
    struct TestRng;

    impl SeedableRng for TestRng {
        type Seed = [u8; 32];

        fn from_seed(seed: Self::Seed) -> Self {
            TestRng
        }
    }

    impl TryRngCore for TestRng {
        type Error = ();
        fn try_from_rng<R: RngCore>(rng: &mut R) -> Result<Self, Self::Error> {
            Ok(TestRng)
        }
    }

    let reseeder = TestRng;
    let result = ReseedingCore::<TestRng, TestRng>::new(100, reseeder);
    assert!(result.is_ok());
    let core = result.unwrap();
    assert_eq!(core.threshold, 100);
}

#[test]
fn test_reseeding_core_new_exceeds_max_threshold() {
    struct TestRng;

    impl SeedableRng for TestRng {
        type Seed = [u8; 32];

        fn from_seed(seed: Self::Seed) -> Self {
            TestRng
        }
    }

    impl TryRngCore for TestRng {
        type Error = ();
        fn try_from_rng<R: RngCore>(rng: &mut R) -> Result<Self, Self::Error> {
            Ok(TestRng)
        }
    }

    let reseeder = TestRng;
    let result = ReseedingCore::<TestRng, TestRng>::new(u64::MAX, reseeder);
    assert!(result.is_ok());
    let core = result.unwrap();
    assert_eq!(core.threshold, i64::MAX);
}

