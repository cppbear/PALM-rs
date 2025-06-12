// Answer 0

#[derive(Default)]
struct TestRng {
    current: usize,
}

impl Rng for TestRng {
    fn random_range(&mut self, range: std::ops::Range<usize>) -> usize {
        let result = self.current % (range.end - range.start) + range.start;
        self.current += 1;
        result
    }
}

#[test]
fn test_sample_array_valid_case() {
    let mut rng = TestRng::default();
    let result = sample_array::<TestRng, 3>(&mut rng, 5);
    assert!(result.is_some());
    let indices = result.unwrap();
    assert_eq!(indices.len(), 3);
    assert!(indices.iter().all(|&x| x < 5));
}

#[test]
fn test_sample_array_exceeds_length() {
    let mut rng = TestRng::default();
    let result = sample_array::<TestRng, 6>(&mut rng, 5);
    assert!(result.is_none());
}

#[test]
fn test_sample_array_exact_length() {
    let mut rng = TestRng::default();
    let result = sample_array::<TestRng, 5>(&mut rng, 5);
    assert!(result.is_some());
    let indices = result.unwrap();
    assert_eq!(indices.len(), 5);
    assert!(indices.iter().all(|&x| x < 5));
}

#[test]
fn test_sample_array_zero_length() {
    let mut rng = TestRng::default();
    let result: Option<[usize; 0]> = sample_array::<TestRng, 0>(&mut rng, 0);
    assert!(result.is_some());
}

