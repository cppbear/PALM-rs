// Answer 0

#[test]
fn test_fmt_non_empty_set() {
    struct TestSet {
        iter: Vec<(i32, ())>,
    }

    impl TestSet {
        fn new() -> Self {
            TestSet { iter: vec![(1, ()), (2, ()), (3, ())] }
        }
        
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            let entries_iter = self.iter.iter().map(|(k, _)| k);
            f.debug_list().entries(entries_iter).finish()
        }
    }

    let set = TestSet::new();
    let result = format!("{:?}", set);
    assert_eq!(result, "[1, 2, 3]");
}

#[test]
fn test_fmt_empty_set() {
    struct TestSet {
        iter: Vec<(i32, ())>,
    }

    impl TestSet {
        fn new() -> Self {
            TestSet { iter: vec![] }
        }
        
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            let entries_iter = self.iter.iter().map(|(k, _)| k);
            f.debug_list().entries(entries_iter).finish()
        }
    }

    let set = TestSet::new();
    let result = format!("{:?}", set);
    assert_eq!(result, "[]");
}

