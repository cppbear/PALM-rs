// Answer 0

#[derive(Debug)]
struct FreqyPacked {
    lcp: FreqyPacked,
}

impl FreqyPacked {
    fn new() -> Self {
        FreqyPacked {
            lcp: FreqyPacked::new(),
        }
    }

    pub fn lcp(&self) -> &FreqyPacked {
        &self.lcp
    }
}

#[test]
fn test_lcp() {
    let freqy = FreqyPacked::new();
    let lcp_result = freqy.lcp();
    assert_eq!(lcp_result, &freqy.lcp);
}

#[test]
fn test_lcp_is_not_null() {
    let freqy = FreqyPacked::new();
    let lcp_result = freqy.lcp();
    assert!(lcp_result as *const _ != std::ptr::null());
}

