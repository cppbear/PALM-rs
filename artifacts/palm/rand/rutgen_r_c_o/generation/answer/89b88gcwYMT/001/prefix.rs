// Answer 0

#[test]
fn test_advance_mdelta_greater_than_zero() {
    let mut rng = Lcg64Xsh32::new(1, 1);
    rng.advance(1);
}

#[test]
fn test_advance_mdelta_large_value() {
    let mut rng = Lcg64Xsh32::new(1, 1);
    rng.advance(18446744073709551615);
}

#[test]
fn test_advance_mdelta_odd_value() {
    let mut rng = Lcg64Xsh32::new(2, 2);
    rng.advance(3);
}

#[test]
#[should_panic] 
fn test_advance_mdelta_zero() {
    let mut rng = Lcg64Xsh32::new(3, 3);
    rng.advance(0);
}

