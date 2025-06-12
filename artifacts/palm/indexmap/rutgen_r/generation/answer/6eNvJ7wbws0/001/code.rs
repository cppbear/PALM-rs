// Answer 0

#[test]
fn test_fmt_with_empty_iterator() {
    struct DummyBucket;

    impl DummyBucket {
        fn refs(&self) -> String {
            String::from("empty")
        }
    }

    struct TestIter {
        iter: Vec<DummyBucket>,
    }

    impl TestIter {
        fn new() -> Self {
            TestIter { iter: Vec::new() }
        }
        
        fn iter(&self) -> &[DummyBucket] {
            &self.iter
        }
        
        fn add_bucket(&mut self, bucket: DummyBucket) {
            self.iter.push(bucket);
        }
    }
    
    let mut test_iter = TestIter::new();
    let fmt_output = format!("{:?}", test_iter);
    assert_eq!(fmt_output, "[]");
}

#[test]
fn test_fmt_with_single_bucket() {
    struct DummyBucket;

    impl DummyBucket {
        fn refs(&self) -> String {
            String::from("single")
        }
    }

    struct TestIter {
        iter: Vec<DummyBucket>,
    }

    impl TestIter {
        fn new() -> Self {
            TestIter { iter: Vec::new() }
        }
        
        fn iter(&self) -> &[DummyBucket] {
            &self.iter
        }
        
        fn add_bucket(&mut self, bucket: DummyBucket) {
            self.iter.push(bucket);
        }
    }
    
    let mut test_iter = TestIter::new();
    test_iter.add_bucket(DummyBucket);
    let fmt_output = format!("{:?}", test_iter);
    assert_eq!(fmt_output, "[single]");
}

#[test]
fn test_fmt_with_multiple_buckets() {
    struct DummyBucket;

    impl DummyBucket {
        fn refs(&self) -> String {
            String::from("bucket")
        }
    }

    struct TestIter {
        iter: Vec<DummyBucket>,
    }

    impl TestIter {
        fn new() -> Self {
            TestIter { iter: Vec::new() }
        }
        
        fn iter(&self) -> &[DummyBucket] {
            &self.iter
        }
        
        fn add_bucket(&mut self, bucket: DummyBucket) {
            self.iter.push(bucket);
        }
    }
    
    let mut test_iter = TestIter::new();
    test_iter.add_bucket(DummyBucket);
    test_iter.add_bucket(DummyBucket);
    let fmt_output = format!("{:?}", test_iter);
    assert_eq!(fmt_output, "[bucket, bucket]");
}

