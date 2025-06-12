// Answer 0

#[test]
fn test_sample_array_with_equal_length() {
    use rand::Rng;
    use rand::rngs::ThreadRng;

    struct TestRng(ThreadRng);

    impl Rng for TestRng {
        fn random_range(&mut self, range: std::ops::Range<usize>) -> usize {
            self.0.gen_range(range)
        }
    }

    let mut rng = TestRng(rand::thread_rng());
    let len = 5;
    let n = len; // Constraint N == len
    let result = sample_array(&mut rng, len);
    
    assert!(result.is_some());
    let indices = result.unwrap();
    let unique_indices: std::collections::HashSet<_> = indices.iter().cloned().collect();
    
    assert_eq!(unique_indices.len(), n); // Ensure all indices are distinct
    assert!(indices.iter().all(|&x| x < len)); // Ensure all indices are within bounds
}

#[test]
fn test_sample_array_beyond_length() {
    use rand::Rng;
    use rand::rngs::ThreadRng;

    struct TestRng(ThreadRng);

    impl Rng for TestRng {
        fn random_range(&mut self, range: std::ops::Range<usize>) -> usize {
            self.0.gen_range(range)
        }
    }

    let mut rng = TestRng(rand::thread_rng());
    let len = 5;
    let n = len + 1; // Constraint N > len
    let result = sample_array(&mut rng, len);
    
    assert!(result.is_none()); // Ensure result is None when N > len
}

#[test]
fn test_sample_array_no_elements() {
    use rand::Rng;
    use rand::rngs::ThreadRng;

    struct TestRng(ThreadRng);

    impl Rng for TestRng {
        fn random_range(&mut self, range: std::ops::Range<usize>) -> usize {
            self.0.gen_range(range)
        }
    }

    let mut rng = TestRng(rand::thread_rng());
    let len = 0;
    let n = 0; // Constraint N == len
    let result = sample_array(&mut rng, len);
    
    assert!(result.is_some());
    assert_eq!(result.unwrap(), []); // Ensure the returned array is empty
}

