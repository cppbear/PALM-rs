// Answer 0

#[test]
fn test_fmt_with_non_empty_iter() {
    struct Bucket {
        value: i32,
    }

    impl Bucket {
        fn value_ref(&self) -> &i32 {
            &self.value
        }
    }

    struct TestIter {
        iter: Vec<Bucket>,
    }

    impl TestIter {
        fn new(iter: Vec<Bucket>) -> Self {
            TestIter { iter }
        }

        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            let iter = self.iter.iter().map(Bucket::value_ref);
            f.debug_list().entries(iter).finish()
        }
    }

    let buckets = vec![Bucket { value: 1 }, Bucket { value: 2 }, Bucket { value: 3 }];
    let test_iter = TestIter::new(buckets);

    let mut output = String::new();
    {
        let mut formatter = std::fmt::Formatter::new(&mut output);
        test_iter.fmt(&mut formatter).unwrap();
    }

    assert_eq!(output, "[1, 2, 3]");
}

#[test]
fn test_fmt_with_empty_iter() {
    struct Bucket {
        value: i32,
    }

    impl Bucket {
        fn value_ref(&self) -> &i32 {
            &self.value
        }
    }

    struct TestIter {
        iter: Vec<Bucket>,
    }

    impl TestIter {
        fn new(iter: Vec<Bucket>) -> Self {
            TestIter { iter }
        }

        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            let iter = self.iter.iter().map(Bucket::value_ref);
            f.debug_list().entries(iter).finish()
        }
    }

    let test_iter = TestIter::new(vec![]);

    let mut output = String::new();
    {
        let mut formatter = std::fmt::Formatter::new(&mut output);
        test_iter.fmt(&mut formatter).unwrap();
    }

    assert_eq!(output, "[]");
}

