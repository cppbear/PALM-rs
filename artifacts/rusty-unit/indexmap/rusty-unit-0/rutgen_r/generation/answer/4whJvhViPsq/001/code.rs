// Answer 0

#[test]
fn test_fmt_empty() {
    struct TestStruct;

    impl Clone for TestStruct {
        fn clone(&self) -> Self {
            TestStruct
        }
    }

    impl std::fmt::Debug for TestStruct {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.debug_list().finish()
        }
    }

    let test_instance = TestStruct;
    let mut buffer = String::new();
    let result = write!(&mut buffer, "{:?}", test_instance);
    assert!(result.is_ok());
    assert_eq!(buffer, "[]");
}

#[test]
fn test_fmt_single_element() {
    struct TestStruct {
        value: i32,
    }

    impl Clone for TestStruct {
        fn clone(&self) -> Self {
            TestStruct { value: self.value }
        }
    }

    impl std::fmt::Debug for TestStruct {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.debug_list().entry(self.value).finish()
        }
    }

    let test_instance = TestStruct { value: 42 };
    let mut buffer = String::new();
    let result = write!(&mut buffer, "{:?}", test_instance);
    assert!(result.is_ok());
    assert_eq!(buffer, "[42]");
}

#[test]
fn test_fmt_multiple_elements() {
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

    impl std::fmt::Debug for TestStruct {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            let mut debug_list = f.debug_list();
            for value in &self.values {
                debug_list.entry(value);
            }
            debug_list.finish()
        }
    }

    let test_instance = TestStruct {
        values: vec![1, 2, 3],
    };
    let mut buffer = String::new();
    let result = write!(&mut buffer, "{:?}", test_instance);
    assert!(result.is_ok());
    assert_eq!(buffer, "[1, 2, 3]");
} 

#[test]
#[should_panic]
fn test_fmt_panic_on_clone() {
    struct PanicOnClone;

    impl Clone for PanicOnClone {
        fn clone(&self) -> Self {
            panic!("Cloning is not allowed");
        }
    }

    impl std::fmt::Debug for PanicOnClone {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.debug_list().entries(self.clone()).finish()
        }
    }

    let test_instance = PanicOnClone;
    let _ = write!(String::new(), "{:?}", test_instance);
}

