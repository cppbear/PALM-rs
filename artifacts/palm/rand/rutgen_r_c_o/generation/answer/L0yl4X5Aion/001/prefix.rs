// Answer 0

#[test]
fn test_advance_positive_mdelta() {
    let mut rng = Lcg128Xsl64::new(42, 7);
    rng.advance(1);
}

#[test]
fn test_advance_large_mdelta() {
    let mut rng = Lcg128Xsl64::new(100, 50);
    rng.advance(u128::MAX);
}

#[test]
fn test_advance_odd_mdelta() {
    let mut rng = Lcg128Xsl64::new(12345, 6789);
    rng.advance(3);
}

#[test]
fn test_advance_zero_mdelta() {
    let mut rng = Lcg128Xsl64::new(1, 1);
    rng.advance(0);
}

