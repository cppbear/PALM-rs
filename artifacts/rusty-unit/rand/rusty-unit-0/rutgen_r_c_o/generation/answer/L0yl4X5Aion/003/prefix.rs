// Answer 0

#[test]
fn test_advance_zero() {
    let mut rng = Lcg128Xsl64::new(12345, 67890);
    rng.advance(0);
}

#[test]
fn test_advance_one() {
    let mut rng = Lcg128Xsl64::new(1, 1);
    rng.advance(1);
}

#[test]
fn test_advance_small() {
    let mut rng = Lcg128Xsl64::new(10, 10);
    rng.advance(10);
}

#[test]
fn test_advance_large() {
    let mut rng = Lcg128Xsl64::new(1000, 2000);
    rng.advance(1000);
}

#[test]
fn test_advance_mid_range() {
    let mut rng = Lcg128Xsl64::new(500, 2500);
    rng.advance(500);
}

#[test]
fn test_advance_max_possible() {
    let mut rng = Lcg128Xsl64::new(u128::MAX, 1);
    rng.advance(u128::MAX);
}

