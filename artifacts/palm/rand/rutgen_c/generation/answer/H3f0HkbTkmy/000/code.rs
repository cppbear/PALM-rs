// Answer 0

#[test]
fn test_sample_single_inclusive_valid_range() {
    struct MockRng {
        value: u32,
    }
    
    impl RngCore for MockRng {
        fn next_u32(&mut self) -> u32 {
            self.value
        }
        
        fn next_u64(&mut self) -> u64 {
            self.value as u64
        }
        
        fn fill_bytes(&mut self, dest: &mut [u8]) {
            for byte in dest.iter_mut() {
                *byte = self.value as u8;
            }
        }
        
        fn ramdon_f32(&mut self) -> f32 {
            self.value as f32
        }
    }
    
    struct MockSampler;

    impl UniformSampler for MockSampler {
        type X = u32;

        fn sample_single_inclusive<R: RngCore + ?Sized>(low: Self::X, high: Self::X, rng: &mut R) -> Result<Self::X, Error> {
            if low > high {
                return Err(Error::EmptyRange);
            }
            Ok(low + rng.next_u32() % (high - low + 1))
        }
    }

    impl SampleUniform for u32 {
        type Sampler = MockSampler;
    }
    
    let rng = &mut MockRng { value: 10 };
    let range_inclusive = 5..=15;
    
    let result = range_inclusive.sample_single(rng).unwrap();
    assert!(result >= 5 && result <= 15);
}

#[test]
fn test_sample_single_inclusive_empty_range() {
    struct MockRng {
        value: u32,
    }
    
    impl RngCore for MockRng {
        fn next_u32(&mut self) -> u32 {
            0
        }
        
        fn next_u64(&mut self) -> u64 {
            0
        }
        
        fn fill_bytes(&mut self, dest: &mut [u8]) {
            for byte in dest.iter_mut() {
                *byte = 0;
            }
        }
        
        fn ramdon_f32(&mut self) -> f32 {
            0.0
        }
    }
    
    struct MockSampler;

    impl UniformSampler for MockSampler {
        type X = u32;

        fn sample_single_inclusive<R: RngCore + ?Sized>(low: Self::X, high: Self::X, rng: &mut R) -> Result<Self::X, Error> {
            if low > high {
                return Err(Error::EmptyRange);
            }
            Ok(low + rng.next_u32() % (high - low + 1))
        }
    }

    impl SampleUniform for u32 {
        type Sampler = MockSampler;
    }
    
    let rng = &mut MockRng { value: 0 };
    let range_inclusive = 10..=5;
    
    let result = range_inclusive.sample_single(rng);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), Error::EmptyRange);
}

#[test]
fn test_sample_single_inclusive_single_value() {
    struct MockRng {
        value: u32,
    }
    
    impl RngCore for MockRng {
        fn next_u32(&mut self) -> u32 {
            0
        }
        
        fn next_u64(&mut self) -> u64 {
            0
        }
        
        fn fill_bytes(&mut self, dest: &mut [u8]) {
            for byte in dest.iter_mut() {
                *byte = 0;
            }
        }
        
        fn ramdon_f32(&mut self) -> f32 {
            0.0
        }
    }
    
    struct MockSampler;

    impl UniformSampler for MockSampler {
        type X = u32;

        fn sample_single_inclusive<R: RngCore + ?Sized>(low: Self::X, high: Self::X, rng: &mut R) -> Result<Self::X, Error> {
            if low > high {
                return Err(Error::EmptyRange);
            }
            Ok(low + rng.next_u32() % (high - low + 1))
        }
    }

    impl SampleUniform for u32 {
        type Sampler = MockSampler;
    }
    
    let rng = &mut MockRng { value: 0 };
    let range_inclusive = 10..=10;
    
    let result = range_inclusive.sample_single(rng).unwrap();
    assert_eq!(result, 10);
}

