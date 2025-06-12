// Answer 0

#[test]
fn test_advance_mdelta_one() {
    let mut rng = Lcg128Xsl64::new(12345, 67890);
    rng.advance(1);
}

#[test]
fn test_advance_mdelta_two() {
    let mut rng = Lcg128Xsl64::new(12345, 67890);
    rng.advance(2);
}

#[test]
fn test_advance_mdelta_four() {
    let mut rng = Lcg128Xsl64::new(12345, 67890);
    rng.advance(4);
}

#[test]
fn test_advance_mdelta_eight() {
    let mut rng = Lcg128Xsl64::new(12345, 67890);
    rng.advance(8);
}

#[test]
fn test_advance_mdelta_sixteen() {
    let mut rng = Lcg128Xsl64::new(12345, 67890);
    rng.advance(16);
}

#[test]
fn test_advance_mdelta_thirty_two() {
    let mut rng = Lcg128Xsl64::new(12345, 67890);
    rng.advance(32);
}

#[test]
fn test_advance_mdelta_zero() {
    let mut rng = Lcg128Xsl64::new(12345, 67890);
    rng.advance(0);
}

