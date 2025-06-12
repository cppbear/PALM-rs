// Answer 0

#[test]
fn test_sample_array_exact_length() {
    use rand::Rng;
    use rand::thread_rng;

    struct TestRng {
        current: usize,
    }

    impl Rng for TestRng {
        fn random_range(&mut self, range: std::ops::Range<usize>) -> usize {
            self.current = (self.current + 1) % range.end;
            self.current
        }
    }

    let mut rng = TestRng { current: 0 };
    let len = 5;
    let result = sample_array(&mut rng, len);
    
    assert!(result.is_some());
    let indices = result.unwrap();
    let mut unique_indices = std::collections::HashSet::new();
    
    for &index in &indices {
        assert!(index < len);
        unique_indices.insert(index);
    }
    
    assert_eq!(unique_indices.len(), indices.len());
}

#[test]
fn test_sample_array_less_than_n() {
    use rand::Rng;
    use rand::thread_rng;

    struct TestRng {
        current: usize,
    }

    impl Rng for TestRng {
        fn random_range(&mut self, range: std::ops::Range<usize>) -> usize {
            self.current = (self.current + 1) % range.end;
            self.current
        }
    }

    let mut rng = TestRng { current: 0 };
    let len = 3;
    let n = 5;
    let result = sample_array(&mut rng, len);
    
    assert!(result.is_none());
}

#[test]
fn test_sample_array_boundary_case() {
    use rand::Rng;
    use rand::thread_rng;

    struct TestRng {
        current: usize,
    }

    impl Rng for TestRng {
        fn random_range(&mut self, range: std::ops::Range<usize>) -> usize {
            self.current = (self.current + 1) % range.end;
            self.current
        }
    }

    let mut rng = TestRng { current: 0 };
    let len = 4;
    let result = sample_array(&mut rng, len);
    
    assert!(result.is_some());
    let indices = result.unwrap();
    assert_eq!(indices.len(), len);
}

#[test]
#[should_panic]
fn test_sample_array_panic_case() {
    use rand::Rng;
    use rand::thread_rng;

    struct TestRng {
        current: usize,
    }

    impl Rng for TestRng {
        fn random_range(&mut self, range: std::ops::Range<usize>) -> usize {
            self.current = (self.current + 1) % range.end;
            self.current
        }
    }

    let mut rng = TestRng { current: 0 };
    let len = 5;
    let result = sample_array(&mut rng, 6); // Should panic
}

