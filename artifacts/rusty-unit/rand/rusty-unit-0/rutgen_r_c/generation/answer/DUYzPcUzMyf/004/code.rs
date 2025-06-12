// Answer 0

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct DummyWeight(i32);

impl Weight for DummyWeight {
    const ZERO: Self = DummyWeight(0);
    
    fn checked_add_assign(&mut self, v: &Self) -> Result<(), ()> {
        let result = self.0.checked_add(v.0);
        match result {
            Some(val) => {
                self.0 = val;
                Ok(())
            },
            None => Err(()),
        }
    }
}

#[derive(Debug, Clone)]
struct DummySampler;

impl UniformSampler for DummySampler {
    type X = DummyWeight;
}

#[derive(Debug, Clone)]
struct DummySampleUniform;

impl SampleUniform for DummySampleUniform {
    type Sampler = DummySampler;
}

#[test]
fn test_weighted_index_invalid_input() {
    let weights: Vec<DummyWeight> = vec![];
    let result = WeightedIndex::<DummyWeight>::new(weights);
    assert_eq!(result, Err(Error::InvalidInput));
}

#[test]
fn test_weighted_index_invalid_weight() {
    let weights = vec![DummyWeight(-1)];
    let result = WeightedIndex::<DummyWeight>::new(weights);
    assert_eq!(result, Err(Error::InvalidWeight));
}

#[test]
fn test_weighted_index_insufficient_non_zero() {
    let weights = vec![DummyWeight(0)];
    let result = WeightedIndex::<DummyWeight>::new(weights);
    assert_eq!(result, Err(Error::InsufficientNonZero));
}

#[test]
fn test_weighted_index_overflow() {
    let mut total_weight = DummyWeight(i32::MAX);
    let weights = vec![DummyWeight(1)];

    // Create a scenario to trigger overflow
    let result = WeightedIndex::<DummyWeight>::new(std::iter::once(total_weight).chain(weights.into_iter()));
    assert_eq!(result, Err(Error::Overflow));
}

