// Answer 0

#[derive(Debug)]
struct FreqyPacked {
    lcs: FreqyPacked,
}

impl FreqyPacked {
    pub fn new() -> Self {
        FreqyPacked {
            lcs: FreqyPacked::default(),
        }
    }
}

impl Default for FreqyPacked {
    fn default() -> Self {
        FreqyPacked {}
    }
}

impl FreqyPacked {
    pub fn lcs(&self) -> &FreqyPacked {
        &self.lcs
    }
}

#[test]
fn test_lcs_returns_reference() {
    let freqy_packed = FreqyPacked::new();
    let lcs_ref = freqy_packed.lcs();
    assert_eq!(lcs_ref, &freqy_packed.lcs);
}

#[test]
fn test_lcs_default_behavior() {
    let freqy_packed = FreqyPacked::default();
    let lcs_ref = freqy_packed.lcs();
    assert!(lcs_ref.is_empty());
}

