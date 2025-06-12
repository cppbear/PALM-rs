// Answer 0

#[test]
fn test_into_entries_non_empty() {
    struct TestBucket {
        value: i32,
    }

    struct TestVec {
        buckets: Vec<TestBucket>,
    }

    impl TestVec {
        fn into_vec(self: Box<Self>) -> Vec<TestBucket> {
            self.buckets
        }
    }

    let test_vec = TestVec {
        buckets: vec![TestBucket { value: 1 }, TestBucket { value: 2 }],
    };

    let result = test_vec.into_vec().into_entries();
    assert_eq!(result.len(), 2);
    assert_eq!(result[0].value, 1);
    assert_eq!(result[1].value, 2);
}

#[test]
fn test_into_entries_empty() {
    struct TestBucket {
        value: i32,
    }

    struct TestVec {
        buckets: Vec<TestBucket>,
    }

    impl TestVec {
        fn into_vec(self: Box<Self>) -> Vec<TestBucket> {
            self.buckets
        }
    }

    let test_vec = TestVec { buckets: vec![] };

    let result = test_vec.into_vec().into_entries();
    assert_eq!(result.len(), 0);
}

#[should_panic]
fn test_into_entries_panic() {
    struct TestBucket {
        value: i32,
    }

    struct TestVec {
        buckets: Vec<TestBucket>,
    }

    impl TestVec {
        fn into_vec(self: Box<Self>) -> Vec<TestBucket> {
            panic!("Intentional panic for testing");
        }
    }

    let test_vec = TestVec {
        buckets: vec![TestBucket { value: 1 }],
    };

    let _ = test_vec.into_vec().into_entries();
}

