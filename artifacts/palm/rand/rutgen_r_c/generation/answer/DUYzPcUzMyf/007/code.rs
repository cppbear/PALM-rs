// Answer 0


#[derive(Clone, Copy, PartialEq, Eq, Debug)]
struct TestWeight(u32);

impl Weight for TestWeight {
    const ZERO: Self = TestWeight(0);
    
    fn checked_add_assign(&mut self, v: &Self) -> Result<(), ()> {
        let new_value = self.0.checked_add(v.0);
        match new_value {
            Some(val) => {
                self.0 = val;
                Ok(())
            }
            None => Err(()),
        }
    }
}

impl SampleUniform for TestWeight {
    type Sampler = TestSampler;
}

struct TestSampler;

impl UniformSampler for TestSampler {
    type X = TestWeight;

    fn new(low: TestWeight, high: TestWeight) -> Result<Self, ()> {
        if low.0 >= high.0 {
            Err(())
        } else {
            Ok(TestSampler)
        }
    }
}

#[test]
fn test_weighted_index_new_valid() {
    let weights: Vec<TestWeight> = vec![TestWeight(2), TestWeight(3), TestWeight(5)];
    let result = WeightedIndex::new(weights);
    
    assert!(result.is_ok());
    let weighted_index = result.unwrap();
    
    assert_eq!(weighted_index.cumulative_weights.len(), 3);
    assert_eq!(weighted_index.total_weight, TestWeight(10));
}

#[test]
fn test_weighted_index_new_empty() {
    let weights: Vec<TestWeight> = vec![];
    let result = WeightedIndex::new(weights);
    
    assert_eq!(result, Err(Error::InvalidInput));
}

#[test]
fn test_weighted_index_new_negative_weight() {
    let weights: Vec<TestWeight> = vec![TestWeight(2), TestWeight(3), TestWeight(5), TestWeight(u32::MAX)];
    let result = WeightedIndex::new(weights);
    
    assert_eq!(result, Err(Error::Overflow));
}

#[test]
fn test_weighted_index_new_zero_total_weight() {
    let weights: Vec<TestWeight> = vec![TestWeight(0), TestWeight(0)];
    let result = WeightedIndex::new(weights);
    
    assert_eq!(result, Err(Error::InsufficientNonZero));
}

#[test]
fn test_weighted_index_new_invalid_weight() {
    let weights: Vec<TestWeight> = vec![TestWeight(2), TestWeight(3), TestWeight(u32::MAX)];
    let result = WeightedIndex::new(weights);
    
    assert_eq!(result, Err(Error::Overflow));
}


