// Answer 0

#[test]
fn test_fmt_with_empty_iter() {
    use std::fmt;

    struct TestStruct;

    impl Clone for TestStruct {
        fn clone(&self) -> Self {
            TestStruct
        }
    }

    impl fmt::Debug for TestStruct {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_list().finish()
        }
    }

    let test_instance = TestStruct;
    let mut output = String::new();
    let result = write!(&mut output, "{:?}", test_instance);
    assert!(result.is_ok());
    assert_eq!(output, "[]");
}

#[test]
fn test_fmt_with_single_item() {
    use std::fmt;

    struct TestStruct {
        value: i32,
    }

    impl Clone for TestStruct {
        fn clone(&self) -> Self {
            TestStruct { value: self.value }
        }
    }

    impl fmt::Debug for TestStruct {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_list().entry(self.value).finish()
        }
    }

    let test_instance = TestStruct { value: 42 };
    let mut output = String::new();
    let result = write!(&mut output, "{:?}", test_instance);
    assert!(result.is_ok());
    assert_eq!(output, "[42]");
}

#[test]
fn test_fmt_with_multiple_items() {
    use std::fmt;

    struct TestStruct {
        values: Vec<i32>,
    }

    impl Clone for TestStruct {
        fn clone(&self) -> Self {
            TestStruct {
                values: self.values.clone(),
            }
        }
    }

    impl fmt::Debug for TestStruct {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let mut list = f.debug_list();
            for value in &self.values {
                list.entry(*value);
            }
            list.finish()
        }
    }

    let test_instance = TestStruct {
        values: vec![1, 2, 3],
    };
    let mut output = String::new();
    let result = write!(&mut output, "{:?}", test_instance);
    assert!(result.is_ok());
    assert_eq!(output, "[1, 2, 3]");
}

