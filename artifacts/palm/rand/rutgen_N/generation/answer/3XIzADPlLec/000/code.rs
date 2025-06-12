// Answer 0

#[test]
fn test_sample() {
    use rand::Rng;
    use rand::distributions::{Distribution, Uniform};

    struct TestStruct(Uniform<i32>);

    impl TestStruct {
        fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> i32 {
            self.0.sample(rng)
        }
    }

    let distr = TestStruct(Uniform::from(1..10));
    let mut rng = rand::thread_rng();
    let result = distr.sample(&mut rng);
    
    assert!(result >= 1 && result < 10);
}

#[test]
#[should_panic]
fn test_sample_out_of_bounds() {
    use rand::Rng;
    use rand::distributions::{Distribution, Uniform};

    struct TestStruct(Uniform<i32>);

    impl TestStruct {
        fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> i32 {
            self.0.sample(rng)
        }
    }

    let distr = TestStruct(Uniform::from(5..5)); // This creates a 0-width range
    let mut rng = rand::thread_rng();
    distr.sample(&mut rng); // This should panic
}

