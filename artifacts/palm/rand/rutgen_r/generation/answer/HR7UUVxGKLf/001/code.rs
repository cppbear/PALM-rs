// Answer 0

#[test]
fn test_weights_initial_state() {
    struct WeightedIndex {
        weights: Vec<u32>,
    }

    impl WeightedIndex {
        fn new(weights: &[u32]) -> Result<Self, &'static str> {
            Ok(Self {
                weights: weights.to_vec(),
            })
        }

        fn weights(&self) -> WeightedIndexIter {
            WeightedIndexIter {
                weighted_index: self,
                index: 0,
            }
        }
    }

    struct WeightedIndexIter<'a> {
        weighted_index: &'a WeightedIndex,
        index: usize,
    }

    impl<'a> Iterator for WeightedIndexIter<'a> {
        type Item = &'a u32;

        fn next(&mut self) -> Option<Self::Item> {
            if self.index < self.weighted_index.weights.len() {
                let weight = &self.weighted_index.weights[self.index];
                self.index += 1;
                Some(weight)
            } else {
                None
            }
        }
    }

    let weights = [1, 2, 3];
    let dist = WeightedIndex::new(&weights).unwrap();
    let collected_weights: Vec<_> = dist.weights().collect();
    assert_eq!(collected_weights, vec![&1, &2, &3]);
}

#[test]
fn test_weights_after_update() {
    struct WeightedIndex {
        weights: Vec<u32>,
    }

    impl WeightedIndex {
        fn new(weights: &[u32]) -> Result<Self, &'static str> {
            Ok(Self {
                weights: weights.to_vec(),
            })
        }

        fn update_weights(&mut self, updates: &[(usize, &u32)]) -> Result<(), &'static str> {
            for &(index, new_weight) in updates {
                if index >= self.weights.len() {
                    return Err("Index out of bounds");
                }
                self.weights[index] = *new_weight;
            }
            Ok(())
        }

        fn weights(&self) -> WeightedIndexIter {
            WeightedIndexIter {
                weighted_index: self,
                index: 0,
            }
        }
    }

    struct WeightedIndexIter<'a> {
        weighted_index: &'a WeightedIndex,
        index: usize,
    }

    impl<'a> Iterator for WeightedIndexIter<'a> {
        type Item = &'a u32;

        fn next(&mut self) -> Option<Self::Item> {
            if self.index < self.weighted_index.weights.len() {
                let weight = &self.weighted_index.weights[self.index];
                self.index += 1;
                Some(weight)
            } else {
                None
            }
        }
    }

    let weights = [1, 2, 3];
    let mut dist = WeightedIndex::new(&weights).unwrap();
    let collected_weights_before: Vec<_> = dist.weights().collect();
    assert_eq!(collected_weights_before, vec![&1, &2, &3]);
    
    dist.update_weights(&[(0, &2)]).unwrap();
    let collected_weights_after: Vec<_> = dist.weights().collect();
    assert_eq!(collected_weights_after, vec![&2, &2, &3]);
}

