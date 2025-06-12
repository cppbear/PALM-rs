// Answer 0

#[test]
fn test_sample_array_exact_length() {
    struct TestRng {
        counter: usize,
    }

    impl Rng for TestRng {
        fn random_range(&mut self, range: std::ops::Range<usize>) -> usize {
            let result = self.counter % (range.end - range.start) + range.start;
            self.counter += 1;
            result
        }
    }

    let mut rng = TestRng { counter: 0 };
    let len = 5;
    let N = 5;
    let result = sample_array(&mut rng, len);
}

#[test]
fn test_sample_array_less_than_length() {
    struct TestRng {
        counter: usize,
    }

    impl Rng for TestRng {
        fn random_range(&mut self, range: std::ops::Range<usize>) -> usize {
            let result = self.counter % (range.end - range.start) + range.start;
            self.counter += 1;
            result
        }
    }

    let mut rng = TestRng { counter: 0 };
    let len = 5;
    let N = 3;
    let result = sample_array(&mut rng, len);
}

#[test]
fn test_sample_array_two_elements() {
    struct TestRng {
        counter: usize,
    }

    impl Rng for TestRng {
        fn random_range(&mut self, range: std::ops::Range<usize>) -> usize {
            let result = self.counter % (range.end - range.start) + range.start;
            self.counter += 1;
            result
        }
    }

    let mut rng = TestRng { counter: 0 };
    let len = 2;
    let N = 2;
    let result = sample_array(&mut rng, len);
}

#[test]
fn test_sample_array_one_element() {
    struct TestRng {
        counter: usize,
    }

    impl Rng for TestRng {
        fn random_range(&mut self, range: std::ops::Range<usize>) -> usize {
            let result = self.counter % (range.end - range.start) + range.start;
            self.counter += 1;
            result
        }
    }

    let mut rng = TestRng { counter: 0 };
    let len = 1;
    let N = 1;
    let result = sample_array(&mut rng, len);
}

#[test]
fn test_sample_array_empty() {
    struct TestRng {
        counter: usize,
    }

    impl Rng for TestRng {
        fn random_range(&mut self, range: std::ops::Range<usize>) -> usize {
            let result = self.counter % (range.end - range.start) + range.start;
            self.counter += 1;
            result
        }
    }

    let mut rng = TestRng { counter: 0 };
    let len = 0;
    let N = 0;
    let result = sample_array(&mut rng, len);
}

