// Answer 0

#[test]
fn test_weight_with_valid_index_zero() {
    struct WeightedIndex {
        cumulative_weights: Vec<u32>,
        total_weight: u32,
    }

    impl WeightedIndex {
        pub fn new(weights: &[u32]) -> Result<Self, &'static str> {
            let total_weight = weights.iter().sum();
            let cumulative_weights = weights.iter().scan(0, |state, &x| {
                *state += x;
                Some(*state)
            }).collect();
            Ok(WeightedIndex {
                cumulative_weights,
                total_weight,
            })
        }

        pub fn weight(&self, index: usize) -> Option<u32> {
            use std::cmp::Ordering::*;

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
    
    assert_eq!(dist.weight(0), Some(0));
    assert_eq!(dist.weight(1), Some(1));
    assert_eq!(dist.weight(2), Some(2));
    assert_eq!(dist.weight(3), None);
}

