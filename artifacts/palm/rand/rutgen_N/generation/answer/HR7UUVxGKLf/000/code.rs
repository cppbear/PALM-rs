// Answer 0

#[test]
fn test_weights_initial() {
    struct WeightedIndex {
        weights: Vec<u32>,
    }

    impl WeightedIndex {
        pub fn new(weights: &[u32]) -> Result<Self, &'static str> {
            Ok(WeightedIndex {
                weights: weights.to_vec(),
            })
        }

        pub fn weights(&self) -> impl Iterator<Item = &u32> {
            self.weights.iter()
        }

        pub fn update_weights(&mut self, updates: &[(usize, &u32)]) -> Result<(), &'static str> {
            for &(index, new_weight) in updates {
                if index < self.weights.len() {
                    self.weights[index] = *new_weight;
                } else {
                    return Err("Index out of bounds");
                }
            }
            Ok(())
        }
    }

    let weights = [1, 2, 3];
    let dist = WeightedIndex::new(&weights).unwrap();
    assert_eq!(dist.weights().collect::<Vec<_>>(), vec![&1, &2, &3]);
}

#[test]
fn test_weights_update() {
    struct WeightedIndex {
        weights: Vec<u32>,
    }

    impl WeightedIndex {
        pub fn new(weights: &[u32]) -> Result<Self, &'static str> {
            Ok(WeightedIndex {
                weights: weights.to_vec(),
            })
        }

        pub fn weights(&self) -> impl Iterator<Item = &u32> {
            self.weights.iter()
        }

        pub fn update_weights(&mut self, updates: &[(usize, &u32)]) -> Result<(), &'static str> {
            for &(index, new_weight) in updates {
                if index < self.weights.len() {
                    self.weights[index] = *new_weight;
                } else {
                    return Err("Index out of bounds");
                }
            }
            Ok(())
        }
    }

    let weights = [1, 2, 3];
    let mut dist = WeightedIndex::new(&weights).unwrap();
    dist.update_weights(&[(0, &2)]).unwrap();
    assert_eq!(dist.weights().collect::<Vec<_>>(), vec![&2, &2, &3]);
}

