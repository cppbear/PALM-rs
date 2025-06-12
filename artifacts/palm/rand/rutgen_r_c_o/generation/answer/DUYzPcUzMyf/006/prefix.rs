// Answer 0

#[test]
fn test_weighted_index_invalid_input_empty() {
    let weights: Vec<f32> = vec![];
    let result = WeightedIndex::new(weights);
}

#[test]
fn test_weighted_index_invalid_weight_negative() {
    let weights: Vec<f32> = vec![-1.0];
    let result = WeightedIndex::new(weights);
}

#[test]
fn test_weighted_index_overflow() {
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    struct TestWeight(u32);
    
    impl Weight for TestWeight {
        const ZERO: Self = TestWeight(0);
        
        fn checked_add_assign(&mut self, v: &Self) -> Result<(), ()> {
            let (new_val, overflowed) = self.0.overflowing_add(v.0);
            if overflowed {
                return Err(());
            }
            self.0 = new_val;
            Ok(())
        }
    }
    
    let weights: Vec<TestWeight> = vec![TestWeight(u32::MAX), TestWeight(1)];
    let result = WeightedIndex::new(weights);
}

#[test]
fn test_weighted_index_insufficient_non_zero() {
    let weights: Vec<f32> = vec![0.0];
    let result = WeightedIndex::new(weights);
}

