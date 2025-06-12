// Answer 0

#[test]
fn test_uniform_new_valid_range_u8() {
    struct DummySampler;
    impl SampleUniform for DummySampler {
        type Sampler = DummySampler;
    }
    
    let result = Uniform::<DummySampler>::new(1u8, 10u8);
    assert!(result.is_ok());
}

#[test]
fn test_uniform_new_valid_range_u16() {
    struct DummySampler;
    impl SampleUniform for DummySampler {
        type Sampler = DummySampler;
    }
    
    let result = Uniform::<DummySampler>::new(100u16, 200u16);
    assert!(result.is_ok());
}

#[test]
fn test_uniform_new_valid_range_u32() {
    struct DummySampler;
    impl SampleUniform for DummySampler {
        type Sampler = DummySampler;
    }
    
    let result = Uniform::<DummySampler>::new(1000u32, 2000u32);
    assert!(result.is_ok());
}

#[test]
fn test_uniform_new_invalid_range_low_greater_than_high() {
    struct DummySampler;
    impl SampleUniform for DummySampler {
        type Sampler = DummySampler;
    }
    
    let result = Uniform::<DummySampler>::new(10u8, 10u8);
    assert!(result.is_err());
}

#[test]
fn test_uniform_new_invalid_range_low_equal_high() {
    struct DummySampler;
    impl SampleUniform for DummySampler {
        type Sampler = DummySampler;
    }
    
    let result = Uniform::<DummySampler>::new(10u8, 5u8);
    assert!(result.is_err());
}

#[test]
fn test_uniform_new_invalid_range_non_finite() {
    struct DummySampler;
    impl SampleUniform for DummySampler {
        type Sampler = DummySampler;
    }
    
    let result = Uniform::<DummySampler>::new(f32::INFINITY, 10.0);
    assert!(result.is_err());
}

#[test]
fn test_uniform_new_boundary_value() {
    struct DummySampler;
    impl SampleUniform for DummySampler {
        type Sampler = DummySampler;
    }
    
    let result = Uniform::<DummySampler>::new(1.0_f32, 2.0_f32);
    assert!(result.is_ok());
}

