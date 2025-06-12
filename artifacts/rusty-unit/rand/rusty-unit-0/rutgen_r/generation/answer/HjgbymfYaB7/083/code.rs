// Answer 0

#[test]
fn test_update_weights_valid() {
    struct WeightStruct {
        cumulative_weights: Vec<f64>,
        total_weight: f64,
        weight_distribution: f64, // Placeholder for sampler
    }

    impl WeightStruct {
        fn new(weights: Vec<f64>) -> Self {
            let total_weight = weights.iter().sum();
            Self {
                cumulative_weights: weights,
                total_weight,
                weight_distribution: 0.0, // Placeholder for actual sampler
            }
        }
    }

    let mut weights = WeightStruct::new(vec![1.0, 2.0, 3.0]);

    let new_weights: Vec<(usize, &f64)> = vec![
        (1, &2.5),  // Update weight at index 1
        (2, &3.5),  // Update weight at index 2
    ];

    let result = weights.update_weights(&new_weights);
    assert_eq!(result, Ok(()));
    assert_eq!(weights.cumulative_weights, vec![1.0, 2.5, 3.5]);
    assert_eq!(weights.total_weight, 7.0);
}

#[test]
#[should_panic(expected = "InvalidInput")]
fn test_update_weights_invalid_index_order() {
    struct WeightStruct {
        cumulative_weights: Vec<f64>,
        total_weight: f64,
        weight_distribution: f64,
    }

    impl WeightStruct {
        fn new(weights: Vec<f64>) -> Self {
            let total_weight = weights.iter().sum();
            Self {
                cumulative_weights: weights,
                total_weight,
                weight_distribution: 0.0,
            }
        }
    }

    let mut weights = WeightStruct::new(vec![1.0, 2.0, 3.0]);

    let new_weights: Vec<(usize, &f64)> = vec![
        (2, &2.5),  // Valid index
        (1, &3.5),  // Invalid index order, should panic
    ];

    let _ = weights.update_weights(&new_weights);
}

#[test]
#[should_panic(expected = "InvalidWeight")]
fn test_update_weights_negative_weight() {
    struct WeightStruct {
        cumulative_weights: Vec<f64>,
        total_weight: f64,
        weight_distribution: f64,
    }

    impl WeightStruct {
        fn new(weights: Vec<f64>) -> Self {
            let total_weight = weights.iter().sum();
            Self {
                cumulative_weights: weights,
                total_weight,
                weight_distribution: 0.0,
            }
        }
    }

    let mut weights = WeightStruct::new(vec![1.0, 2.0, 3.0]);

    let new_weights: Vec<(usize, &f64)> = vec![
        (1, &-1.0),  // Negative weight, should panic
    ];

    let _ = weights.update_weights(&new_weights);
}

#[test]
#[should_panic(expected = "InsufficientNonZero")]
fn test_update_weights_insufficient_non_zero() {
    struct WeightStruct {
        cumulative_weights: Vec<f64>,
        total_weight: f64,
        weight_distribution: f64,
    }

    impl WeightStruct {
        fn new(weights: Vec<f64>) -> Self {
            let total_weight = weights.iter().sum();
            Self {
                cumulative_weights: weights,
                total_weight,
                weight_distribution: 0.0,
            }
        }
    }

    let mut weights = WeightStruct::new(vec![0.0, 0.0, 0.0]);

    let new_weights: Vec<(usize, &f64)> = vec![
        (0, &0.0),  // Updates that still lead to total weight of 0, should panic
    ];

    let _ = weights.update_weights(&new_weights);
}

