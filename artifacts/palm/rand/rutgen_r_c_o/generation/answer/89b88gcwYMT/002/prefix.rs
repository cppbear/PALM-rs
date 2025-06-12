// Answer 0

#[test]
fn test_advance_mdelta_zero() {
    let mut rng = Lcg64Xsh32::new(42, 7);
    rng.advance(0);
}

#[test]
fn test_advance_mdelta_one() {
    let mut rng = Lcg64Xsh32::new(42, 7);
    rng.advance(1);
}

#[test]
fn test_advance_mdelta_two() {
    let mut rng = Lcg64Xsh32::new(42, 7);
    rng.advance(2);
}

#[test]
fn test_advance_mdelta_three() {
    let mut rng = Lcg64Xsh32::new(42, 7);
    rng.advance(3);
}

#[test]
fn test_advance_mdelta_four() {
    let mut rng = Lcg64Xsh32::new(42, 7);
    rng.advance(4);
}

#[test]
fn test_advance_mdelta_five() {
    let mut rng = Lcg64Xsh32::new(42, 7);
    rng.advance(5);
}

#[test]
fn test_advance_mdelta_eight() {
    let mut rng = Lcg64Xsh32::new(42, 7);
    rng.advance(8);
}

#[test]
fn test_advance_mdelta_nine() {
    let mut rng = Lcg64Xsh32::new(42, 7);
    rng.advance(9);
}

#[test]
fn test_advance_mdelta_sixteen() {
    let mut rng = Lcg64Xsh32::new(42, 7);
    rng.advance(16);
}

#[test]
fn test_advance_mdelta_seventeen() {
    let mut rng = Lcg64Xsh32::new(42, 7);
    rng.advance(17);
}

