// Answer 0

#[test]
fn test_fmt() {
    use std::fmt;

    #[derive(Clone)]
    struct TestStruct {
        data: Vec<i32>,
    }

    impl TestStruct {
        fn new(data: Vec<i32>) -> Self {
            TestStruct { data }
        }
    }

    impl fmt::Debug for TestStruct {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_list().entries(self.data.iter()).finish()
        }
    }

    let test_instance = TestStruct::new(vec![1, 2, 3]);
    let expected_output = format!("{:?}", test_instance.clone());
    let mut output_buffer = String::new();
    let result = write!(&mut output_buffer, "{:?}", test_instance);

    assert!(result.is_ok());
    assert_eq!(output_buffer, expected_output);
}

#[test]
fn test_fmt_empty() {
    use std::fmt;

    #[derive(Clone)]
    struct TestStruct {
        data: Vec<i32>,
    }

    impl TestStruct {
        fn new(data: Vec<i32>) -> Self {
            TestStruct { data }
        }
    }

    impl fmt::Debug for TestStruct {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_list().entries(self.data.iter()).finish()
        }
    }

    let test_instance = TestStruct::new(vec![]);
    let expected_output = format!("{:?}", test_instance.clone());
    let mut output_buffer = String::new();
    let result = write!(&mut output_buffer, "{:?}", test_instance);

    assert!(result.is_ok());
    assert_eq!(output_buffer, expected_output);
}

