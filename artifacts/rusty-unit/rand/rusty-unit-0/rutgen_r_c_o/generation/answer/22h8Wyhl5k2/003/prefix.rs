// Answer 0

#[test]
fn test_advance_with_mdelta_zero() {
    let mut rng = Mcg128Xsl64::new(42);
    rng.advance(0);
}

#[test]
fn test_advance_with_mdelta_zero_edge_case() {
    let mut rng = Mcg128Xsl64::new(u128::MAX);
    rng.advance(0);
}

