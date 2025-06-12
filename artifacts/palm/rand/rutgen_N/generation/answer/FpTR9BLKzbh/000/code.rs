// Answer 0

#[derive(Debug, Clone)]
struct Distribution<X> {
    total_weight: X,
}

impl<X: Clone> Distribution<X> {
    pub fn new(total_weight: X) -> Self {
        Distribution { total_weight }
    }

    pub fn total_weight(&self) -> X {
        self.total_weight.clone()
    }
}

#[test]
fn test_total_weight() {
    let distribution = Distribution::new(10);
    assert_eq!(distribution.total_weight(), 10);
}

#[test]
fn test_total_weight_clone() {
    let distribution = Distribution::new(5);
    let weight = distribution.total_weight();
    assert_eq!(weight, 5);
    let new_weight = weight + 5; // Ensure cloning allows independent modification
    assert_eq!(new_weight, 10);
    assert_eq!(distribution.total_weight(), 5);
}

