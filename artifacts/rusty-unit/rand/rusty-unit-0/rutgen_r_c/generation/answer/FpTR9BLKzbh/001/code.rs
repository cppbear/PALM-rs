// Answer 0

#[test]
fn test_total_weight_non_empty() {
    use crate::distr::uniform::UniformSampler;
    use crate::Rng;

    struct TestSampler;

    impl UniformSampler for TestSampler {
        type X = f64;
    }

    struct TestRng;

    impl Rng for TestRng {
        // Implement the necessary methods for the Rng trait.
    }

    let weights = vec![1.0, 2.0, 3.0];
    let total_weight = weights.iter().sum();

    let weighted_index = WeightedIndex {
        cumulative_weights: weights.clone(),
        total_weight,
        weight_distribution: TestSampler,
    };

    assert_eq!(weighted_index.total_weight(), total_weight);
}

#[test]
fn test_total_weight_empty() {
    use crate::distr::uniform::UniformSampler;
    use crate::Rng;

    struct TestSampler;

    impl UniformSampler for TestSampler {
        type X = f64;
    }

    struct TestRng;

    impl Rng for TestRng {
        // Implement the necessary methods for the Rng trait.
    }

    let weighted_index = WeightedIndex {
        cumulative_weights: Vec::new(),
        total_weight: 0.0,
        weight_distribution: TestSampler,
    };

    assert_eq!(weighted_index.total_weight(), 0.0);
}

