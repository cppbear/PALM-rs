// Answer 0

#[test]
fn test_update_weights_success() {
    use core::ops::{AddAssign, SubAssign};

    struct TestWeight(u32);
    
    impl Weight for TestWeight {}

    impl SampleUniform for TestWeight {
        type Sampler = TestSampler;
    }

    struct TestSampler {
        min: TestWeight,
        max: TestWeight,
    }

    impl UniformSampler for TestSampler {
        type X = TestWeight;

        fn new(min: TestWeight, max: TestWeight) -> Result<Self, Error> {
            Ok(TestSampler { min, max })
        }
    }

    let mut weighted_index = WeightedIndex {
        cumulative_weights: vec![TestWeight(1), TestWeight(2), TestWeight(3)],
        total_weight: TestWeight(6),
        weight_distribution: TestSampler::new(TestWeight(0), TestWeight(6)).unwrap(),
    };

    let new_weights: Vec<(usize, &TestWeight)> = vec![(1, &TestWeight(0)), (2, &TestWeight(1))];

    let result = weighted_index.update_weights(&new_weights);

    assert_eq!(result, Ok(()));
    assert_eq!(weighted_index.cumulative_weights, vec![TestWeight(1), TestWeight(1), TestWeight(2)]);
    assert_eq!(weighted_index.total_weight, TestWeight(3));
}

#[test]
fn test_update_weights_empty_new_weights() {
    struct TestWeight(u32);
    
    impl Weight for TestWeight {}

    impl SampleUniform for TestWeight {
        type Sampler = TestSampler;
    }

    struct TestSampler {
        min: TestWeight,
        max: TestWeight,
    }

    impl UniformSampler for TestSampler {
        type X = TestWeight;

        fn new(min: TestWeight, max: TestWeight) -> Result<Self, Error> {
            Ok(TestSampler { min, max })
        }
    }

    let mut weighted_index = WeightedIndex {
        cumulative_weights: vec![TestWeight(1), TestWeight(2), TestWeight(3)],
        total_weight: TestWeight(6),
        weight_distribution: TestSampler::new(TestWeight(0), TestWeight(6)).unwrap(),
    };

    let new_weights: Vec<(usize, &TestWeight)> = vec![];

    let result = weighted_index.update_weights(&new_weights);

    assert_eq!(result, Ok(()));
    assert_eq!(weighted_index.cumulative_weights, vec![TestWeight(1), TestWeight(2), TestWeight(3)]);
    assert_eq!(weighted_index.total_weight, TestWeight(6));
}

#[test]
#[should_panic]
fn test_update_weights_invalid_index() {
    struct TestWeight(u32);
    
    impl Weight for TestWeight {}

    impl SampleUniform for TestWeight {
        type Sampler = TestSampler;
    }

    struct TestSampler {
        min: TestWeight,
        max: TestWeight,
    }

    impl UniformSampler for TestSampler {
        type X = TestWeight;

        fn new(min: TestWeight, max: TestWeight) -> Result<Self, Error> {
            Ok(TestSampler { min, max })
        }
    }

    let mut weighted_index = WeightedIndex {
        cumulative_weights: vec![TestWeight(1), TestWeight(2), TestWeight(3)],
        total_weight: TestWeight(6),
        weight_distribution: TestSampler::new(TestWeight(0), TestWeight(6)).unwrap(),
    };

    let new_weights: Vec<(usize, &TestWeight)> = vec![(3, &TestWeight(4))]; // invalid index

    let _ = weighted_index.update_weights(&new_weights);
}

#[test]
#[should_panic]
fn test_update_weights_negative_weight() {
    struct TestWeight(i32);
    
    impl Weight for TestWeight {}

    impl SampleUniform for TestWeight {
        type Sampler = TestSampler;
    }

    struct TestSampler {
        min: TestWeight,
        max: TestWeight,
    }

    impl UniformSampler for TestSampler {
        type X = TestWeight;

        fn new(min: TestWeight, max: TestWeight) -> Result<Self, Error> {
            Ok(TestSampler { min, max })
        }
    }

    let mut weighted_index = WeightedIndex {
        cumulative_weights: vec![TestWeight(1), TestWeight(2), TestWeight(3)],
        total_weight: TestWeight(6),
        weight_distribution: TestSampler::new(TestWeight(0), TestWeight(6)).unwrap(),
    };

    let new_weights: Vec<(usize, &TestWeight)> = vec![(1, &TestWeight(-1))]; // negative weight

    let _ = weighted_index.update_weights(&new_weights);
}

#[test]
#[should_panic]
fn test_update_weights_insufficient_non_zero() {
    struct TestWeight(u32);
    
    impl Weight for TestWeight {}

    impl SampleUniform for TestWeight {
        type Sampler = TestSampler;
    }

    struct TestSampler {
        min: TestWeight,
        max: TestWeight,
    }

    impl UniformSampler for TestSampler {
        type X = TestWeight;

        fn new(min: TestWeight, max: TestWeight) -> Result<Self, Error> {
            Ok(TestSampler { min, max })
        }
    }

    let mut weighted_index = WeightedIndex {
        cumulative_weights: vec![TestWeight(1), TestWeight(0), TestWeight(0)],
        total_weight: TestWeight(1),
        weight_distribution: TestSampler::new(TestWeight(0), TestWeight(1)).unwrap(),
    };

    let new_weights: Vec<(usize, &TestWeight)> = vec![(0, &TestWeight(0))]; // results in total_weight <= zero

    let _ = weighted_index.update_weights(&new_weights);
}

