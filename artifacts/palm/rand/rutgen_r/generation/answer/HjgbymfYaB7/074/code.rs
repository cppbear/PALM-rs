// Answer 0

#[test]
fn test_update_weights_valid() {
    struct TestWeights {
        cumulative_weights: Vec<f64>,
        total_weight: f64,
        weight_distribution: (),
    }

    impl TestWeights {
        fn new(cumulative_weights: Vec<f64>) -> Self {
            let total_weight = cumulative_weights.last().copied().unwrap_or(0.0);
            Self {
                cumulative_weights,
                total_weight,
                weight_distribution: (),
            }
        }
    }

    let mut weights = TestWeights::new(vec![1.0, 2.0, 3.0]);
    let new_weights: Vec<(usize, &f64)> = vec![(0, &0.0), (1, &4.0)];

    let result = weights.update_weights(&new_weights);
    assert_eq!(result, Ok(()));
}

#[test]
fn test_update_weights_invalid_input() {
    struct TestWeights {
        cumulative_weights: Vec<f64>,
        total_weight: f64,
        weight_distribution: (),
    }

    impl TestWeights {
        fn new(cumulative_weights: Vec<f64>) -> Self {
            let total_weight = cumulative_weights.last().copied().unwrap_or(0.0);
            Self {
                cumulative_weights,
                total_weight,
                weight_distribution: (),
            }
        }
    }

    let mut weights = TestWeights::new(vec![1.0, 2.0, 3.0]);
    let new_weights: Vec<(usize, &f64)> = vec![(1, &2.0), (0, &3.0)];

    let result = weights.update_weights(&new_weights);
    assert_eq!(result, Err(Error::InvalidInput));
}

#[test]
fn test_update_weights_invalid_weight() {
    struct TestWeights {
        cumulative_weights: Vec<f64>,
        total_weight: f64,
        weight_distribution: (),
    }

    impl TestWeights {
        fn new(cumulative_weights: Vec<f64>) -> Self {
            let total_weight = cumulative_weights.last().copied().unwrap_or(0.0);
            Self {
                cumulative_weights,
                total_weight,
                weight_distribution: (),
            }
        }
    }

    let mut weights = TestWeights::new(vec![1.0, 2.0, 3.0]);
    let new_weights: Vec<(usize, &f64)> = vec![(0, &-1.0)];

    let result = weights.update_weights(&new_weights);
    assert_eq!(result, Err(Error::InvalidWeight));
}

#[test]
fn test_update_weights_insufficient_nonzero() {
    struct TestWeights {
        cumulative_weights: Vec<f64>,
        total_weight: f64,
        weight_distribution: (),
    }

    impl TestWeights {
        fn new(cumulative_weights: Vec<f64>) -> Self {
            let total_weight = cumulative_weights.last().copied().unwrap_or(0.0);
            Self {
                cumulative_weights,
                total_weight,
                weight_distribution: (),
            }
        }
    }

    let mut weights = TestWeights::new(vec![0.0, 0.0, 0.0]);
    let new_weights: Vec<(usize, &f64)> = vec![(0, &0.0)];

    let result = weights.update_weights(&new_weights);
    assert_eq!(result, Err(Error::InsufficientNonZero));
}

