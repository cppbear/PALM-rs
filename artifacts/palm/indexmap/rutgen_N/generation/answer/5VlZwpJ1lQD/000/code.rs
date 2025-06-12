// Answer 0

#[test]
fn test_len_empty() {
    struct TestIndices {
        indices: Vec<usize>,
    }

    impl TestIndices {
        fn len(&self) -> usize {
            self.indices.len()
        }
    }

    let test_indices = TestIndices { indices: Vec::new() };
    assert_eq!(test_indices.len(), 0);
}

#[test]
fn test_len_non_empty() {
    struct TestIndices {
        indices: Vec<usize>,
    }

    impl TestIndices {
        fn len(&self) -> usize {
            self.indices.len()
        }
    }

    let test_indices = TestIndices { indices: vec![1, 2, 3] };
    assert_eq!(test_indices.len(), 3);
}

