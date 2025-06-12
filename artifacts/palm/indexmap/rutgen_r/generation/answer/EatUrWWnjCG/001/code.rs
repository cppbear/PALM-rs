// Answer 0

#[test]
fn test_fmt_with_empty_iter() {
    struct FakeBucket;
    impl FakeBucket {
        fn refs(&self) -> usize {
            0
        }
    }

    struct TestStruct {
        iter: Vec<FakeBucket>,
    }

    impl TestStruct {
        fn new() -> Self {
            TestStruct { iter: Vec::new() }
        }

        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            let iter = self.iter.iter().map(FakeBucket::refs);
            f.debug_list().entries(iter).finish()
        }
    }

    let test_struct = TestStruct::new();
    let result = format!("{:?}", test_struct.fmt(&mut std::fmt::Formatter::new()));
    assert_eq!(result, "[]");
}

#[test]
fn test_fmt_with_single_element() {
    struct FakeBucket;
    impl FakeBucket {
        fn refs(&self) -> usize {
            10
        }
    }

    struct TestStruct {
        iter: Vec<FakeBucket>,
    }

    impl TestStruct {
        fn new() -> Self {
            TestStruct { iter: vec![FakeBucket] }
        }

        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            let iter = self.iter.iter().map(FakeBucket::refs);
            f.debug_list().entries(iter).finish()
        }
    }

    let test_struct = TestStruct::new();
    let result = format!("{:?}", test_struct.fmt(&mut std::fmt::Formatter::new()));
    assert_eq!(result, "[10]");
}

#[test]
fn test_fmt_with_multiple_elements() {
    struct FakeBucket;
    impl FakeBucket {
        fn refs(&self) -> usize {
            5
        }
    }

    struct TestStruct {
        iter: Vec<FakeBucket>,
    }

    impl TestStruct {
        fn new() -> Self {
            TestStruct { iter: vec![FakeBucket, FakeBucket] }
        }

        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            let iter = self.iter.iter().map(FakeBucket::refs);
            f.debug_list().entries(iter).finish()
        }
    }

    let test_struct = TestStruct::new();
    let result = format!("{:?}", test_struct.fmt(&mut std::fmt::Formatter::new()));
    assert_eq!(result, "[5, 5]");
}

