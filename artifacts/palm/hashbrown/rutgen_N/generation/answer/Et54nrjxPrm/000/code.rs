// Answer 0

#[test]
fn test_fmt() {
    use std::fmt;

    struct TestStruct {
        data: Vec<i32>,
    }

    impl TestStruct {
        fn new(data: Vec<i32>) -> Self {
            TestStruct { data }
        }
    }

    impl Clone for TestStruct {
        fn clone(&self) -> Self {
            TestStruct { data: self.data.clone() }
        }
    }

    impl fmt::Debug for TestStruct {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_list().entries(self.clone().data.iter()).finish()
        }
    }

    let test_instance = TestStruct::new(vec![1, 2, 3]);
    let result = format!("{:?}", test_instance);
    assert_eq!(result, "[1, 2, 3]");
}

