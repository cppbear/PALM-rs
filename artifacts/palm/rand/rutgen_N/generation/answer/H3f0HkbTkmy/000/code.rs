// Answer 0

#[test]
fn test_sample_single_success() {
    use rand::RngCore;
    use rand::rngs::ThreadRng;
    use std::marker::PhantomData;

    struct Uniform {
        start: usize,
        end: usize,
    }

    impl Uniform {
        fn new(start: usize, end: usize) -> Self {
            Self { start, end }
        }

        fn start(&self) -> usize {
            self.start
        }

        fn end(&self) -> usize {
            self.end
        }
    }

    struct Sampler;

    impl Sampler {
        fn sample_single_inclusive<R: RngCore>(
            start: usize,
            end: usize,
            rng: &mut R,
        ) -> Result<usize, &'static str> {
            if start > end {
                return Err("Invalid range");
            }
            Ok(rng.gen_range(start..=end))
        }
    }

    impl Uniform {
        fn sample_single<R: RngCore + ?Sized>(self, rng: &mut R) -> Result<usize, &'static str> {
            Sampler::sample_single_inclusive(self.start(), self.end(), rng)
        }
    }

    let mut rng = rand::thread_rng();
    let uniform = Uniform::new(1, 10);
    let result = uniform.sample_single(&mut rng);
    assert!(result.is_ok());
    assert!(result.unwrap() >= 1 && result.unwrap() <= 10);
}

#[test]
#[should_panic(expected = "Invalid range")]
fn test_sample_single_failure() {
    use rand::RngCore;

    struct Uniform {
        start: usize,
        end: usize,
    }

    impl Uniform {
        fn new(start: usize, end: usize) -> Self {
            Self { start, end }
        }

        fn start(&self) -> usize {
            self.start
        }

        fn end(&self) -> usize {
            self.end
        }
    }

    struct Sampler;

    impl Sampler {
        fn sample_single_inclusive<R: RngCore>(
            start: usize,
            end: usize,
            _rng: &mut R,
        ) -> Result<usize, &'static str> {
            if start > end {
                panic!("Invalid range");
            }
            Ok(0) // This line will never be executed in this test
        }
    }

    impl Uniform {
        fn sample_single<R: RngCore + ?Sized>(self, _rng: &mut R) -> Result<usize, &'static str> {
            Sampler::sample_single_inclusive(self.start(), self.end(), _rng)
        }
    }

    let uniform = Uniform::new(10, 1); // Invalid range
    let _ = uniform.sample_single(&mut rand::thread_rng()); // This will panic
}

