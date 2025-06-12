// Answer 0

#[test]
fn test_update_weights_valid_case() {
    struct DummyWeight;
    impl SampleUniform for DummyWeight {
        type Sampler = DummySampler;
    }

    struct DummySampler;
    impl UniformSampler for DummySampler {
        type X = DummyWeight;
        fn new(_low: Self::X, _high: Self::X) -> Result<Self, Error> {
            Ok(DummySampler)
        }
    }

    let mut weighted_index = WeightedIndex {
        cumulative_weights: vec![0.0, 1.0, 4.0],
        total_weight: 5.0,
        weight_distribution: DummySampler::new(0.0, 5.0).unwrap(),
    };

    let new_weights = vec![(1, &2.0), (2, &3.0)];
    weighted_index.update_weights(&new_weights).unwrap();
}

#[test]
fn test_update_weights_multiple_updates() {
    struct DummyWeight;
    impl SampleUniform for DummyWeight {
        type Sampler = DummySampler;
    }

    struct DummySampler;
    impl UniformSampler for DummySampler {
        type X = DummyWeight;
        fn new(_low: Self::X, _high: Self::X) -> Result<Self, Error> {
            Ok(DummySampler)
        }
    }

    let mut weighted_index = WeightedIndex {
        cumulative_weights: vec![0.0, 1.0, 4.0, 7.0],
        total_weight: 8.0,
        weight_distribution: DummySampler::new(0.0, 8.0).unwrap(),
    };

    let new_weights = vec![(1, &2.0), (2, &3.0), (3, &1.0)];
    weighted_index.update_weights(&new_weights).unwrap();
}

#[test]
fn test_update_weights_edge_case_no_change() {
    struct DummyWeight;
    impl SampleUniform for DummyWeight {
        type Sampler = DummySampler;
    }

    struct DummySampler;
    impl UniformSampler for DummySampler {
        type X = DummyWeight;
        fn new(_low: Self::X, _high: Self::X) -> Result<Self, Error> {
            Ok(DummySampler)
        }
    }

    let mut weighted_index = WeightedIndex {
        cumulative_weights: vec![0.0, 1.0, 4.0],
        total_weight: 5.0,
        weight_distribution: DummySampler::new(0.0, 5.0).unwrap(),
    };

    let new_weights: Vec<(usize, &f64)> = vec![];
    weighted_index.update_weights(&new_weights).unwrap();
}

#[test]
fn test_update_weights_with_no_non_negative() {
    struct DummyWeight;
    impl SampleUniform for DummyWeight {
        type Sampler = DummySampler;
    }

    struct DummySampler;
    impl UniformSampler for DummySampler {
        type X = DummyWeight;
        fn new(_low: Self::X, _high: Self::X) -> Result<Self, Error> {
            Ok(DummySampler)
        }
    }

    let mut weighted_index = WeightedIndex {
        cumulative_weights: vec![0.0, 1.0, 4.0],
        total_weight: 5.0,
        weight_distribution: DummySampler::new(0.0, 5.0).unwrap(),
    };

    let new_weights = vec![(1, &0.0), (2, &-3.0)];
    let result = weighted_index.update_weights(&new_weights);
    match result {
        Err(Error::InvalidWeight) => (),
        _ => panic!("Expected InvalidWeight error"),
    }
}

