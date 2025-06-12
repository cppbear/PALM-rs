// Answer 0

#[test]
fn test_fmt_with_empty_iter() {
    struct MockBucket {
        key: usize,
    }

    impl MockBucket {
        fn key_ref(&self) -> &usize {
            &self.key
        }
    }

    struct MockIter {
        iter: Vec<MockBucket>,
    }

    impl MockIter {
        fn as_slice(&self) -> &[MockBucket] {
            &self.iter
        }
    }

    let empty_iter = MockIter { iter: vec![] };
    let mut output = String::new();
    let result = write!(output, "{:?}", empty_iter);
    assert!(result.is_ok());
    assert_eq!(output, "[]");
}

#[test]
fn test_fmt_with_single_element_iter() {
    struct MockBucket {
        key: usize,
    }

    impl MockBucket {
        fn key_ref(&self) -> &usize {
            &self.key
        }
    }

    struct MockIter {
        iter: Vec<MockBucket>,
    }

    impl MockIter {
        fn as_slice(&self) -> &[MockBucket] {
            &self.iter
        }
    }

    let single_element_iter = MockIter { iter: vec![MockBucket { key: 1 }] };
    let mut output = String::new();
    let result = write!(output, "{:?}", single_element_iter);
    assert!(result.is_ok());
    assert_eq!(output, "[1]");
}

#[test]
fn test_fmt_with_multiple_elements_iter() {
    struct MockBucket {
        key: usize,
    }

    impl MockBucket {
        fn key_ref(&self) -> &usize {
            &self.key
        }
    }

    struct MockIter {
        iter: Vec<MockBucket>,
    }

    impl MockIter {
        fn as_slice(&self) -> &[MockBucket] {
            &self.iter
        }
    }

    let multi_element_iter = MockIter { iter: vec![MockBucket { key: 1 }, MockBucket { key: 2 }, MockBucket { key: 3 }] };
    let mut output = String::new();
    let result = write!(output, "{:?}", multi_element_iter);
    assert!(result.is_ok());
    assert_eq!(output, "[1, 2, 3]");
}

