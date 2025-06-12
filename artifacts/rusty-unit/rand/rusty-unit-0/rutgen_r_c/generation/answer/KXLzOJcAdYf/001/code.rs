// Answer 0

#[test]
fn test_uniform_new_valid_range() {
    struct SampleStruct;

    impl SampleUniform for SampleStruct {
        type Sampler = SampleStructSampler;
    }

    struct SampleStructSampler;

    impl UniformSampler for SampleStructSampler {
        type X = SampleStruct;

        fn new<B1, B2>(_low: B1, _high: B2) -> Result<Self, Error> {
            Ok(SampleStructSampler)
        }
        
        fn sample_single(_low: usize, _high: usize, _rng: &mut dyn RngCore) -> Result<SampleStruct, Error> {
            Ok(SampleStruct)
        }
    }

    let result = Uniform::<SampleStruct>::new(0u32, 10u32);
    assert!(result.is_ok());
}

#[test]
#[should_panic]
fn test_uniform_new_invalid_range_low_greater_than_high() {
    struct SampleStruct;

    impl SampleUniform for SampleStruct {
        type Sampler = SampleStructSampler;
    }

    struct SampleStructSampler;

    impl UniformSampler for SampleStructSampler {
        type X = SampleStruct;

        fn new<B1, B2>(_low: B1, _high: B2) -> Result<Self, Error> {
            Err(Error::EmptyRange)
        }

        fn sample_single(_low: usize, _high: usize, _rng: &mut dyn RngCore) -> Result<SampleStruct, Error> {
            Ok(SampleStruct)
        }
    }

    let _ = Uniform::<SampleStruct>::new(10u32, 0u32);
}

#[test]
#[should_panic]
fn test_uniform_new_equal_low_high() {
    struct SampleStruct;

    impl SampleUniform for SampleStruct {
        type Sampler = SampleStructSampler;
    }

    struct SampleStructSampler;

    impl UniformSampler for SampleStructSampler {
        type X = SampleStruct;

        fn new<B1, B2>(_low: B1, _high: B2) -> Result<Self, Error> {
            Err(Error::EmptyRange)
        }

        fn sample_single(_low: usize, _high: usize, _rng: &mut dyn RngCore) -> Result<SampleStruct, Error> {
            Ok(SampleStruct)
        }
    }

    let _ = Uniform::<SampleStruct>::new(5u32, 5u32);
}

#[test]
#[should_panic]
fn test_uniform_new_non_finite_values() {
    struct SampleStruct;

    impl SampleUniform for SampleStruct {
        type Sampler = SampleStructSampler;
    }

    struct SampleStructSampler;

    impl UniformSampler for SampleStructSampler {
        type X = SampleStruct;

        fn new<B1, B2>(_low: B1, _high: B2) -> Result<Self, Error> {
            Err(Error::NonFinite)
        }

        fn sample_single(_low: usize, _high: usize, _rng: &mut dyn RngCore) -> Result<SampleStruct, Error> {
            Ok(SampleStruct)
        }
    }

    let _ = Uniform::<SampleStruct>::new(f32::INFINITY, f32::MAX);
}

