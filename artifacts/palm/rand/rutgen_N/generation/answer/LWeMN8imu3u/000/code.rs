// Answer 0

#[test]
fn test_sample_with_valid_rng() {
    use rand::Rng;
    use rand::thread_rng;

    struct TestDistribution;

    impl TestDistribution {
        fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> u32 {
            rng.gen_range(1..100)
        }
    }

    let mut rng = thread_rng();
    let distribution = TestDistribution;
    let result = distribution.sample(&mut rng);

    assert!(result >= 1 && result < 100);
}

#[test]
#[should_panic]
fn test_sample_with_rng_failure() {
    use rand::Rng;
    use rand::thread_rng;

    struct TestDistribution;

    impl TestDistribution {
        fn sample<R: Rng + ?Sized>(&self, _rng: &mut R) -> u32 {
            panic!("Forced panic for testing.");
        }
    }

    let mut rng = thread_rng();
    let distribution = TestDistribution;

    distribution.sample(&mut rng);
}

