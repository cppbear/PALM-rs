// Answer 0

#[derive(Debug, Clone, PartialEq)]
struct TestSampler;

impl SampleUniform for TestSampler {
    type Sampler = TestSampler;
}

impl WeightedIndex<TestSampler> {
    pub fn new(weights: &[u32]) -> Result<Self, Error> {
        let cumulative_weights: Vec<u32> = weights.to_vec();
        let total_weight = cumulative_weights.iter().sum();
        Ok(WeightedIndex {
            cumulative_weights,
            total_weight,
            weight_distribution: TestSampler,
        })
    }
}

#[test]
fn test_weight_index_out_of_bounds() {
    let weights = [0, 1, 2];
    let dist = WeightedIndex::new(&weights).unwrap();
    let index = 4; // greater than the length of cumulative_weights (which is 3)
    let result = dist.weight(index);
}

#[test]
fn test_weight_index_greater_than_bound() {
    let weights = [10, 20, 30];
    let dist = WeightedIndex::new(&weights).unwrap();
    let index = 5; // greater than the length of cumulative_weights (which is 3)
    let result = dist.weight(index);
}

