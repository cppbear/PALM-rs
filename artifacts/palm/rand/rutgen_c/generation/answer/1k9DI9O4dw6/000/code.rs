// Answer 0

#[test]
fn test_mcg128xsl64_new() {
    let state = 0xcafef00dd15ea5e5;
    let rng = Mcg128Xsl64::new(state);
    assert_eq!(rng.state, state | 1);
}

#[test]
fn test_mcg128xsl64_new_zero() {
    let state = 0;
    let rng = Mcg128Xsl64::new(state);
    assert_eq!(rng.state, state | 1);
}

#[test]
fn test_mcg128xsl64_new_max() {
    let state = u128::MAX;
    let rng = Mcg128Xsl64::new(state);
    assert_eq!(rng.state, state | 1);
}

