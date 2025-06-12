// Answer 0

#[test]
fn test_fmt() {
    struct Bucket {
        value: usize,
    }

    impl Bucket {
        fn refs(&self) -> &usize {
            &self.value
        }
    }

    struct TestIter {
        iter: Vec<Bucket>,
    }

    impl TestIter {
        fn new(iter: Vec<Bucket>) -> Self {
            Self { iter }
        }

        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            let iter = self.iter.as_slice().iter().map(Bucket::refs);
            f.debug_list().entries(iter).finish()
        }
    }

    let buckets = vec![Bucket { value: 1 }, Bucket { value: 2 }, Bucket { value: 3 }];
    let test_iter = TestIter::new(buckets);

    let result = format!("{:?}", test_iter);
    assert!(result.contains("1") && result.contains("2") && result.contains("3"));
}

