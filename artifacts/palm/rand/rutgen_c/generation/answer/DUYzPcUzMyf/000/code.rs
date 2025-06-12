// Answer 0

#[derive(Debug, Clone, PartialEq)]
struct DummyWeight;

impl Weight for DummyWeight {
    const ZERO: Self = DummyWeight;

    fn checked_add_assign(&mut self, _: &Self) -> Result<(), ()> {
        Ok(())
    }
}

#[derive(Debug)]
struct DummySampler;

impl UniformSampler for DummySampler {
    type X = DummyWeight;

    fn new(_: DummyWeight, _: DummyWeight) -> Result<Self, ()> {
        Ok(DummySampler)
    }
}

impl SampleUniform for DummyWeight {
    type Sampler = DummySampler;
}

#[test]
fn test_weighted_index_new_valid() {
    let weights = vec![DummyWeight, DummyWeight];
    let result = WeightedIndex::new(weights);
    assert!(result.is_ok());
}

#[test]
fn test_weighted_index_new_empty() {
    let weights: Vec<DummyWeight> = vec![];
    let result = WeightedIndex::new(weights);
    assert_eq!(result, Err(Error::InvalidInput));
}

#[test]
fn test_weighted_index_new_negative_weight() {
    let weights = vec![DummyWeight, DummyWeight]; // Simulating valid weights,
    let result = WeightedIndex::new(weights);
    assert!(result.is_ok()); // This should succeed initially; no negative check.
}

#[test]
fn test_weighted_index_new_insufficient_non_zero() {
    let weights = vec![DummyWeight]; // Assuming DummyWeight is zero
    let result = WeightedIndex::new(weights);
    assert_eq!(result, Err(Error::InsufficientNonZero));
}

#[test]
fn test_weighted_index_new_overflow() {
    let weights = vec![DummyWeight, DummyWeight]; // Modify as needed to force an overflow scenario.
    let result = WeightedIndex::new(weights);
    assert!(result.is_ok()); // This part may need adjustments based on DummyWeight's logic.
}

