// Answer 0

#[test]
fn test_advance_mdelta_greater_than_zero_odd() {
    let mut rng = Mcg128Xsl64::new(42);
    rng.advance(1);
}

#[test]
fn test_advance_mdelta_greater_than_zero_even() {
    let mut rng = Mcg128Xsl64::new(42);
    rng.advance(2);
}

#[test]
#[should_panic]
fn test_advance_mdelta_zero() {
    let mut rng = Mcg128Xsl64::new(42);
    rng.advance(0);
}

