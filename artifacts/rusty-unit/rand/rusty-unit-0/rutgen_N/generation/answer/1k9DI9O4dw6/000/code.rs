// Answer 0

#[derive(Debug)]
struct Mcg128Xsl64 {
    state: u128,
}

impl Mcg128Xsl64 {
    pub fn new(state: u128) -> Self {
        Mcg128Xsl64 { state: state | 1 }
    }
}

#[test]
fn test_new_with_zero_state() {
    let rng = Mcg128Xsl64::new(0);
    assert_eq!(rng.state, 1);
}

#[test]
fn test_new_with_non_zero_state() {
    let rng = Mcg128Xsl64::new(0xcafef00dd15ea5e5);
    assert_eq!(rng.state, 0xcafef00dd15ea5e5 | 1);
}

#[test]
fn test_new_with_large_value() {
    let large_state: u128 = u128::MAX;
    let rng = Mcg128Xsl64::new(large_state);
    assert_eq!(rng.state, u128::MAX | 1);
}

