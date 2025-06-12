// Answer 0

#[test]
fn test_total_weight_non_empty() {
    struct WeightedDistribution {
        total_weight: f64,
    }

    impl WeightedDistribution {
        fn new(total_weight: f64) -> Self {
            Self { total_weight }
        }

        pub fn total_weight(&self) -> f64 {
            self.total_weight.clone()
        }
    }

    let dist = WeightedDistribution::new(42.0);
    assert_eq!(dist.total_weight(), 42.0);
}

#[test]
fn test_total_weight_zero() {
    struct WeightedDistribution {
        total_weight: f64,
    }

    impl WeightedDistribution {
        fn new(total_weight: f64) -> Self {
            Self { total_weight }
        }

        pub fn total_weight(&self) -> f64 {
            self.total_weight.clone()
        }
    }

    let dist = WeightedDistribution::new(0.0);
    assert_eq!(dist.total_weight(), 0.0);
}

#[test]
fn test_total_weight_negative() {
    struct WeightedDistribution {
        total_weight: f64,
    }

    impl WeightedDistribution {
        fn new(total_weight: f64) -> Self {
            Self { total_weight }
        }

        pub fn total_weight(&self) -> f64 {
            self.total_weight.clone()
        }
    }

    let dist = WeightedDistribution::new(-10.0);
    assert_eq!(dist.total_weight(), -10.0);
}

