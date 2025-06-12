// Answer 0

#[test]
fn test_fmt_for_empty_set() {
    struct TestSet {
        data: Vec<i32>,
    }

    impl TestSet {
        fn iter(&self) -> std::slice::Iter<i32> {
            self.data.iter()
        }
    }

    let empty_set = TestSet { data: vec![] };
    let mut output = String::new();
    let result = write!(&mut output, "{:?}", empty_set);
    assert!(result.is_ok());
    assert_eq!(output, "{}");
}

#[test]
fn test_fmt_for_non_empty_set() {
    struct TestSet {
        data: Vec<i32>,
    }

    impl TestSet {
        fn iter(&self) -> std::slice::Iter<i32> {
            self.data.iter()
        }
    }

    let non_empty_set = TestSet { data: vec![1, 2, 3] };
    let mut output = String::new();
    let result = write!(&mut output, "{:?}", non_empty_set);
    assert!(result.is_ok());
    assert_eq!(output, "{1, 2, 3}");
}

