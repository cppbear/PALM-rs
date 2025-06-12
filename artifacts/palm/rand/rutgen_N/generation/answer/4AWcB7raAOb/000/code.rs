// Answer 0

#[derive(Default)]
struct TestRandCore {
    index: usize,
    half_used: bool,
    results: Vec<u8>,
}

impl TestRandCore {
    pub fn reset(&mut self) {
        self.index = self.results.len();
        self.half_used = false;
    }
}

#[test]
fn test_reset_empty_results() {
    let mut rand_core = TestRandCore::default();
    rand_core.reset();
    assert_eq!(rand_core.index, 0);
    assert!(!rand_core.half_used);
}

#[test]
fn test_reset_non_empty_results() {
    let mut rand_core = TestRandCore {
        results: vec![1, 2, 3],
        ..TestRandCore::default()
    };
    rand_core.reset();
    assert_eq!(rand_core.index, 3);
    assert!(!rand_core.half_used);
}

#[test]
fn test_reset_after_use() {
    let mut rand_core = TestRandCore {
        results: vec![1, 2, 3],
        index: 2,
        half_used: true,
    };
    rand_core.reset();
    assert_eq!(rand_core.index, 3);
    assert!(!rand_core.half_used);
}

