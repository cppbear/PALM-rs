// Answer 0

#[test]
fn test_fmt_debug_list() {
    use std::fmt;

    struct TestStruct {
        values: Vec<i32>,
    }

    impl TestStruct {
        fn new(values: Vec<i32>) -> Self {
            TestStruct { values }
        }
    }

    impl Clone for TestStruct {
        fn clone(&self) -> Self {
            TestStruct::new(self.values.clone())
        }
    }

    impl fmt::Debug for TestStruct {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_list().entries(self.clone().values.iter()).finish()
        }
    }

    let test_instance = TestStruct::new(vec![1, 2, 3]);
    let result = format!("{:?}", test_instance);
    assert_eq!(result, "[1, 2, 3]");
}

