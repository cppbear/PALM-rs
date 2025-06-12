// Answer 0

#[test]
fn test_weighted_index_new_invalid_weight() {
    use alloc::vec::Vec;
    use core::marker::PhantomData;

    #[derive(Clone)]
    struct TestWeight;
    
    impl Weight for TestWeight {
        const ZERO: Self = TestWeight;
        fn checked_add_assign(&mut self, _v: &Self) -> Result<(), ()> {
            Err(()) // Mocking overflow
        }
    }

    // Mocking the SampleUniform trait
    struct TestSampler;
    
    impl SampleUniform for TestWeight {
        type Sampler = TestSampler;
    }

    impl TestSampler {
        pub fn new(_low: TestWeight, _high: TestWeight) -> Result<Self, ()> {
            Ok(TestSampler) // Simplified constructor
        }
    }

    let weights: Vec<TestWeight> = vec![TestWeight, TestWeight]; // All weights are valid
    let result = WeightedIndex::<TestWeight>::new(weights);
    
    assert_eq!(result, Err(Error::InvalidWeight)); // Expect invalid weight due to checked_add_assign
}

#[test]
fn test_weighted_index_new_insufficient_non_zero() {
    use alloc::vec::Vec;

    #[derive(Clone)]
    struct TestWeight;
    
    impl Weight for TestWeight {
        const ZERO: Self = TestWeight;

        fn checked_add_assign(&mut self, _v: &Self) -> Result<(), ()> {
            Ok(()) // No overflow
        }
    }

    // Mocking the SampleUniform trait
    struct TestSampler;
    
    impl SampleUniform for TestWeight {
        type Sampler = TestSampler;
    }

    impl TestSampler {
        pub fn new(_low: TestWeight, _high: TestWeight) -> Result<Self, ()> {
            Ok(TestSampler) // Simplified constructor
        }
    }

    let weights: Vec<TestWeight> = vec![TestWeight]; // Only one weight which is zero
    let result = WeightedIndex::<TestWeight>::new(weights);
    
    assert_eq!(result, Err(Error::InsufficientNonZero)); // Expect insufficient non-zero weights
}

