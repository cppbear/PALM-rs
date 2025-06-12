// Answer 0

#[test]
fn test_sample_always_true() {
    use rand::Rng;

    struct Bernoulli {
        p_int: u64,
    }

    impl Bernoulli {
        fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> bool {
            if self.p_int == u64::MAX {
                return true;
            }
            let v: u64 = rng.gen(); // Using a random number generator
            v < self.p_int
        }
    }

    let mut rng = rand::thread_rng();
    let bernoulli = Bernoulli { p_int: u64::MAX }; // ALWAYS_TRUE

    assert_eq!(bernoulli.sample(&mut rng), true);
}

