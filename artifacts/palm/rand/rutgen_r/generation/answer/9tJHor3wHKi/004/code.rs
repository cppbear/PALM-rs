// Answer 0

#[test]
fn test_weight_valid_index() {
    struct WeightedIndex {
        cumulative_weights: Vec<i32>,
        total_weight: i32,
    }

    impl WeightedIndex {
        pub fn new(weights: &[i32]) -> Option<Self> {
            let cumulative_weights: Vec<i32> = weights.iter()
                .scan(0, |state, &x| {
                    *state += x;
                    Some(*state)
                }).collect();
            let total_weight = cumulative_weights.last().cloned().unwrap_or(0);
            Some(Self { cumulative_weights, total_weight })
        }

        pub fn weight(&self, index: usize) -> Option<i32> {
            use core::cmp::Ordering::*;

            let mut weight = match index.cmp(&self.cumulative_weights.len()) {
                Less => self.cumulative_weights[index].clone(),
                Equal => self.total_weight.clone(),
                Greater => return None,
            };

            if index > 0 {
                weight -= &self.cumulative_weights[index - 1];
            }
            Some(weight)
        }
    }

    let weights = [0, 1, 2, 3];
    let dist = WeightedIndex::new(&weights).unwrap();

    assert_eq!(dist.weight(0), Some(0));
    assert_eq!(dist.weight(1), Some(1));
    assert_eq!(dist.weight(2), Some(2));
    assert_eq!(dist.weight(3), Some(3));
}

#[test]
fn test_weight_index_out_of_bounds() {
    struct WeightedIndex {
        cumulative_weights: Vec<i32>,
        total_weight: i32,
    }

    impl WeightedIndex {
        pub fn new(weights: &[i32]) -> Option<Self> {
            let cumulative_weights: Vec<i32> = weights.iter()
                .scan(0, |state, &x| {
                    *state += x;
                    Some(*state)
                }).collect();
            let total_weight = cumulative_weights.last().cloned().unwrap_or(0);
            Some(Self { cumulative_weights, total_weight })
        }

        pub fn weight(&self, index: usize) -> Option<i32> {
            use core::cmp::Ordering::*;

            let mut weight = match index.cmp(&self.cumulative_weights.len()) {
                Less => self.cumulative_weights[index].clone(),
                Equal => self.total_weight.clone(),
                Greater => return None,
            };

            if index > 0 {
                weight -= &self.cumulative_weights[index - 1];
            }
            Some(weight)
        }
    }

    let weights = [1, 2, 3];
    let dist = WeightedIndex::new(&weights).unwrap();

    assert_eq!(dist.weight(3), None);
}

#[test]
fn test_weight_index_greater_than_length() {
    struct WeightedIndex {
        cumulative_weights: Vec<i32>,
        total_weight: i32,
    }

    impl WeightedIndex {
        pub fn new(weights: &[i32]) -> Option<Self> {
            let cumulative_weights: Vec<i32> = weights.iter()
                .scan(0, |state, &x| {
                    *state += x;
                    Some(*state)
                }).collect();
            let total_weight = cumulative_weights.last().cloned().unwrap_or(0);
            Some(Self { cumulative_weights, total_weight })
        }

        pub fn weight(&self, index: usize) -> Option<i32> {
            use core::cmp::Ordering::*;

            let mut weight = match index.cmp(&self.cumulative_weights.len()) {
                Less => self.cumulative_weights[index].clone(),
                Equal => self.total_weight.clone(),
                Greater => return None,
            };

            if index > 0 {
                weight -= &self.cumulative_weights[index - 1];
            }
            Some(weight)
        }
    }

    let weights = [0, 1, 2, 3, 4];
    let dist = WeightedIndex::new(&weights).unwrap();

    assert_eq!(dist.weight(5), None);
}

