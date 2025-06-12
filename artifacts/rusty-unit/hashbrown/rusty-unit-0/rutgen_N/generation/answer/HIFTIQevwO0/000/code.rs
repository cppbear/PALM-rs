// Answer 0

#[test]
fn test_fmt_with_empty_inner() {
    use std::fmt;

    struct TestStruct {
        inner: Vec<i32>,
    }

    impl TestStruct {
        fn new() -> Self {
            TestStruct { inner: Vec::new() }
        }

        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_list()
                .entries(self.inner.iter())
                .finish()
        }
    }

    let test_struct = TestStruct::new();
    let result = format!("{:?}", test_struct);
    assert_eq!(result, "[]");
}

#[test]
fn test_fmt_with_non_empty_inner() {
    use std::fmt;

    struct TestStruct {
        inner: Vec<i32>,
    }

    impl TestStruct {
        fn new(inner: Vec<i32>) -> Self {
            TestStruct { inner }
        }

        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_list()
                .entries(self.inner.iter())
                .finish()
        }
    }

    let test_struct = TestStruct::new(vec![1, 2, 3]);
    let result = format!("{:?}", test_struct);
    assert_eq!(result, "[1, 2, 3]");
}

