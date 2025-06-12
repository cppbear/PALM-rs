// Answer 0

#[derive(Default)]
struct Results {
    index: usize,
    half_used: bool,
    results: Vec<u8>,
}

impl Results {
    pub fn reset(&mut self) {
        self.index = self.results.len();
        self.half_used = false;
    }
}

#[test]
fn test_reset_empty_results() {
    let mut results = Results::default();
    results.reset();
    assert_eq!(results.index, 0);
    assert_eq!(results.half_used, false);
}

#[test]
fn test_reset_non_empty_results() {
    let mut results = Results {
        results: vec![1, 2, 3],
        ..Default::default()
    };
    results.reset();
    assert_eq!(results.index, 3);
    assert_eq!(results.half_used, false);
}

#[test]
fn test_reset_with_half_used() {
    let mut results = Results {
        index: 1,
        half_used: true,
        results: vec![5, 6],
    };
    results.reset();
    assert_eq!(results.index, 2);
    assert_eq!(results.half_used, false);
}

