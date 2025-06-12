// Answer 0

#[test]
fn test_fmt_empty_slice() {
    struct TestSlice {
        data: Vec<i32>,
    }

    impl std::fmt::Debug for TestSlice {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.debug_list().entries(&self.data).finish()
        }
    }

    let slice = TestSlice { data: vec![] };
    let result = format!("{:?}", slice);
    assert_eq!(result, "[]");
}

#[test]
fn test_fmt_non_empty_slice() {
    struct TestSlice {
        data: Vec<i32>,
    }

    impl std::fmt::Debug for TestSlice {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.debug_list().entries(&self.data).finish()
        }
    }

    let slice = TestSlice { data: vec![1, 2, 3] };
    let result = format!("{:?}", slice);
    assert_eq!(result, "[1, 2, 3]");
}

