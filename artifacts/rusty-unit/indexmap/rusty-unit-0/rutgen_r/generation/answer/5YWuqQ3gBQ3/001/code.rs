// Answer 0

#[test]
fn test_fmt_with_empty_collection() {
    struct TestStruct;

    impl Clone for TestStruct {
        fn clone(&self) -> Self {
            TestStruct
        }
    }

    impl std::fmt::Debug for TestStruct {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str("TestStruct")
        }
    }

    let empty_collection: Vec<TestStruct> = Vec::new();
    let result = format!("{:?}", empty_collection);
    assert_eq!(result, "[]");
}

#[test]
fn test_fmt_with_single_element() {
    struct TestStruct;

    impl Clone for TestStruct {
        fn clone(&self) -> Self {
            TestStruct
        }
    }

    impl std::fmt::Debug for TestStruct {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str("TestStruct")
        }
    }

    let single_element_collection = vec![TestStruct];
    let result = format!("{:?}", single_element_collection);
    assert_eq!(result, "[TestStruct]");
}

#[test]
fn test_fmt_with_multiple_elements() {
    struct TestStruct(i32);

    impl Clone for TestStruct {
        fn clone(&self) -> Self {
            TestStruct(self.0)
        }
    }

    impl std::fmt::Debug for TestStruct {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "TestStruct({})", self.0)
        }
    }

    let multiple_elements_collection = vec![TestStruct(1), TestStruct(2), TestStruct(3)];
    let result = format!("{:?}", multiple_elements_collection);
    assert_eq!(result, "[TestStruct(1), TestStruct(2), TestStruct(3)]");
}

#[test]
#[should_panic]
fn test_fmt_with_panic_condition() {
    struct TestStruct;

    impl Clone for TestStruct {
        fn clone(&self) -> Self {
            panic!("This should panic on clone");
        }
    }

    let collection = vec![TestStruct];
    let _ = format!("{:?}", collection);
}

