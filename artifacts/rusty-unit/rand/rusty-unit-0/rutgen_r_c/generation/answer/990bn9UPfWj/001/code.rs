// Answer 0

#[test]
fn test_new_inclusive_valid_range() {
    struct TestSampler;
    impl SampleUniform for TestSampler {
        type Sampler = Self;
    }
    
    let result = Uniform::<TestSampler>::new_inclusive(1, 10);
    assert!(result.is_ok(), "Expected result to be Ok");
}

#[test]
fn test_new_inclusive_empty_range() {
    struct TestSampler;
    impl SampleUniform for TestSampler {
        type Sampler = Self;
    }
    
    let result = Uniform::<TestSampler>::new_inclusive(10, 1);
    assert!(matches!(result, Err(Error::EmptyRange)), "Expected an EmptyRange error");
}

#[test]
fn test_new_inclusive_non_finite() {
    struct TestSampler;
    impl SampleUniform for TestSampler {
        type Sampler = Self;
    }
    
    let result = Uniform::<TestSampler>::new_inclusive(f32::NAN, 10);
    assert!(matches!(result, Err(Error::NonFinite)), "Expected a NonFinite error");

    let result = Uniform::<TestSampler>::new_inclusive(1, f32::INFINITY);
    assert!(matches!(result, Err(Error::NonFinite)), "Expected a NonFinite error");
}

#[test]
fn test_new_inclusive_equal_bounds() {
    struct TestSampler;
    impl SampleUniform for TestSampler {
        type Sampler = Self;
    }
    
    let result = Uniform::<TestSampler>::new_inclusive(5, 5);
    assert!(result.is_ok(), "Expected result to be Ok");
}

