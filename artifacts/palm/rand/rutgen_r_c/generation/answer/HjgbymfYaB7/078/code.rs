// Answer 0

fn test_update_weights_invalid_weight() {
    struct TestSampler;

    impl UniformSampler for TestSampler {
        type X = f32;
        fn new(low: Self::X, high: Self::X) -> Result<Self, Error> {
            if low < high {
                Ok(TestSampler)
            } else {
                Err(Error::InvalidWeight)
            }
        }
    }

    struct TestWeight {
        value: f32,
    }

    impl SampleUniform for TestWeight {
        type Sampler = TestSampler;
    }

    impl Clone for TestWeight {
        fn clone(&self) -> Self {
            TestWeight { value: self.value }
        }
    }

    impl Default for TestWeight {
        fn default() -> Self {
            TestWeight { value: 0.0 }
        }
    }

    impl PartialOrd for TestWeight {
        fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
            self.value.partial_cmp(&other.value)
        }
    }

    let mut weighted_index = WeightedIndex {
        cumulative_weights: vec![
            TestWeight { value: 1.0 },
            TestWeight { value: 2.0 },
            TestWeight { value: 3.0 },
        ],
        total_weight: TestWeight { value: 6.0 },
        weight_distribution: TestSampler::new(0.0, 6.0).unwrap(),
    };

    let new_weights = vec![(1, &TestWeight { value: -1.0 })];
    let result = weighted_index.update_weights(&new_weights);
    assert_eq!(result, Err(Error::InvalidWeight));
}

