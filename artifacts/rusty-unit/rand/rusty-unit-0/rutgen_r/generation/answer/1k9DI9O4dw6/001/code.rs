// Answer 0

#[derive(Debug)]
struct Mcg128Xsl64 {
    state: u128,
}

impl Mcg128Xsl64 {
    pub fn new(state: u128) -> Self {
        // Force low bit to 1, as in C version.
        Mcg128Xsl64 { state: state | 1 }
    }
}

#[test]
fn test_new_with_zero_state() {
    let expected = Mcg128Xsl64 { state: 1 }; // 0 | 1 = 1
    let result = Mcg128Xsl64::new(0);
    assert_eq!(result, expected);
}

#[test]
fn test_new_with_large_state() {
    let state: u128 = u128::MAX; // Max value for u128
    let expected = Mcg128Xsl64 { state: u128::MAX | 1 }; // Max value | 1 = u128::MAX
    let result = Mcg128Xsl64::new(state);
    assert_eq!(result, expected);
}

#[test]
fn test_new_with_small_odd_state() {
    let state: u128 = 3; // Small odd number
    let expected = Mcg128Xsl64 { state: 3 }; // 3 | 1 = 3 (already odd)
    let result = Mcg128Xsl64::new(state);
    assert_eq!(result, expected);
}

#[test]
fn test_new_with_small_even_state() {
    let state: u128 = 2; // Small even number
    let expected = Mcg128Xsl64 { state: 3 }; // 2 | 1 = 3
    let result = Mcg128Xsl64::new(state);
    assert_eq!(result, expected);
}

#[test]
fn test_new_with_one_state() {
    let state: u128 = 1; // State is already odd
    let expected = Mcg128Xsl64 { state: 1 }; // 1 | 1 = 1
    let result = Mcg128Xsl64::new(state);
    assert_eq!(result, expected);
}

