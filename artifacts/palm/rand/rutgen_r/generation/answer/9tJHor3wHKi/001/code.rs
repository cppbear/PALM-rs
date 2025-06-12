// Answer 0

#[test]
fn test_weight_out_of_bounds_greater() {
    struct WeightedIndex {
        cumulative_weights: Vec<u32>,
        total_weight: u32,
    }

    impl WeightedIndex {
        fn new(weights: &[u32]) -> Result<Self, ()> {
            let cumulative_weights: Vec<u32> = weights.iter().cloned().scan(0, |state, weight| {
                *state += weight;
                Some(*state)
            }).collect();
            let total_weight = cumulative_weights.last().cloned().unwrap_or(0);
            Ok(WeightedIndex {
                cumulative_weights,
                total_weight,
            })
        }

        pub fn weight(&self, index: usize) -> Option<u32> {
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

    let weights = [0, 1, 2];
    let dist = WeightedIndex::new(&weights).unwrap();
    assert_eq!(dist.weight(3), None); // Test with index greater than length
}

