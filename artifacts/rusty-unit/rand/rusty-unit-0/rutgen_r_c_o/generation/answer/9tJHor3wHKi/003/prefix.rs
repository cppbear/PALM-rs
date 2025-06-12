// Answer 0

#[test]
fn test_weight_index_zero() {
    use rand::distr::weighted::WeightedIndex;
    use rand::Rng;

    let weights = vec![10, 20, 30];
    let dist = WeightedIndex::new(&weights).unwrap();
    let result = dist.weight(0);
}

#[test]
fn test_weight_index_equal_to_total_weight() {
    use rand::distr::weighted::WeightedIndex;
    use rand::Rng;

    let weights = vec![10, 20, 30];
    let dist = WeightedIndex::new(&weights).unwrap();
    let result = dist.weight(3);
}

