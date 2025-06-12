// Answer 0

#[cfg(test)]
mod tests {
    use rand::Rng;
    use rand::distributions::{Distribution, Uniform};

    struct SampleStruct(Uniform<i32>);

    impl SampleStruct {
        fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> i32 {
            self.0.sample(rng)
        }
    }

    #[test]
    fn test_sample_valid_range() {
        let dist = Uniform::from(1..10);
        let sample_struct = SampleStruct(dist);
        let mut rng = rand::thread_rng();

        // Run sample multiple times to observe valid outputs
        for _ in 0..10 {
            let value = sample_struct.sample(&mut rng);
            assert!(value >= 1 && value < 10);
        }
    }

    #[test]
    fn test_sample_empty_range() {
        let dist = Uniform::from(10..10);
        let sample_struct = SampleStruct(dist);
        let mut rng = rand::thread_rng();

        // This should panic because the range is empty
        let result = std::panic::catch_unwind(|| {
            sample_struct.sample(&mut rng);
        });
        
        assert!(result.is_err());
    }

    #[test]
    fn test_sample_beyond_max_int_range() {
        let dist = Uniform::from(i32::MIN..i32::MAX);
        let sample_struct = SampleStruct(dist);
        let mut rng = rand::thread_rng();

        // Run sample multiple times to observe valid outputs
        for _ in 0..10 {
            let value = sample_struct.sample(&mut rng);
            assert!(value >= i32::MIN && value <= i32::MAX);
        }
    }
}

