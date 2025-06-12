// Answer 0

#[test]
fn test_fmt_empty_iterator() {
    struct TestBucket {
        key: i32,
    }

    struct TestIter {
        iter: Vec<TestBucket>,
    }

    impl TestIter {
        fn key_ref(&self) -> &i32 {
            &self.iter[0].key
        }

        fn iter(&self) -> &[TestBucket] {
            &self.iter
        }
    }

    let empty_iter = TestIter { iter: vec![] };
    let result = format!("{:?}", empty_iter);
    assert_eq!(result, "[]");
}

#[test]
fn test_fmt_single_element_iterator() {
    struct TestBucket {
        key: i32,
    }

    struct TestIter {
        iter: Vec<TestBucket>,
    }

    impl TestIter {
        fn key_ref(&self) -> &i32 {
            &self.iter[0].key
        }

        fn iter(&self) -> &[TestBucket] {
            &self.iter
        }
    }

    let single_element_iter = TestIter { iter: vec![TestBucket { key: 1 }] };
    let result = format!("{:?}", single_element_iter);
    assert_eq!(result, "[1]");
}

#[test]
fn test_fmt_multiple_elements_iterator() {
    struct TestBucket {
        key: i32,
    }

    struct TestIter {
        iter: Vec<TestBucket>,
    }

    impl TestIter {
        fn key_ref(&self) -> &i32 {
            &self.iter[0].key
        }

        fn iter(&self) -> &[TestBucket] {
            &self.iter
        }
    }

    let multiple_elements_iter = TestIter { iter: vec![TestBucket { key: 1 }, TestBucket { key: 2 }, TestBucket { key: 3 }] };
    let result = format!("{:?}", multiple_elements_iter);
    assert_eq!(result, "[1, 2, 3]");
}

