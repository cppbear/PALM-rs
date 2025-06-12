// Answer 0

#[test]
fn test_fmt_with_non_empty_iter() {
    struct TestBucket {
        key: usize,
    }

    struct TestIter {
        iter: Vec<TestBucket>,
    }

    impl TestIter {
        fn new(keys: Vec<usize>) -> Self {
            let buckets = keys.into_iter().map(|key| TestBucket { key }).collect();
            TestIter { iter: buckets }
        }

        fn key_ref(&self) -> &usize {
            &self.key
        }
    }

    let test_iter = TestIter::new(vec![1, 2, 3]);
    let mut output = String::new();
    let result = std::fmt::Formatter::write(&mut output, &test_iter);
    assert!(result.is_ok());
    assert_eq!(output, "[1, 2, 3]");
}

#[test]
fn test_fmt_with_empty_iter() {
    struct TestBucket {
        key: usize,
    }

    struct TestIter {
        iter: Vec<TestBucket>,
    }

    impl TestIter {
        fn new() -> Self {
            TestIter { iter: Vec::new() }
        }

        fn key_ref(&self) -> &usize {
            &self.key
        }
    }

    let test_iter = TestIter::new();
    let mut output = String::new();
    let result = std::fmt::Formatter::write(&mut output, &test_iter);
    assert!(result.is_ok());
    assert_eq!(output, "[]");
}

#[should_panic]
#[test]
fn test_fmt_with_panic_condition() {
    struct TestBucket {
        key: usize,
    }

    struct TestIter {
        iter: Vec<TestBucket>,
    }

    impl TestIter {
        fn new(keys: Vec<usize>) -> Self {
            let buckets = keys.into_iter().map(|key| TestBucket { key }).collect();
            TestIter { iter: buckets }
        }

        fn key_ref(&self) -> &usize {
            panic!("Intentional panic for testing");
        }
    }

    let test_iter = TestIter::new(vec![1]);
    let _ = test_iter.key_ref(); // This should trigger the panic
}

