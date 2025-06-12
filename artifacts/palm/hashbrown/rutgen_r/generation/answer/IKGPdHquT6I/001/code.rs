// Answer 0

#[test]
fn test_fmt_empty() {
    #[derive(Clone)]
    struct TestStruct;

    impl std::fmt::Debug for TestStruct {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str("TestStruct")
        }
    }

    let test_item: TestStruct = TestStruct;
    let result = format!("{:?}", test_item);
    assert_eq!(result, "TestStruct");
}

#[test]
fn test_fmt_single_entry() {
    #[derive(Clone)]
    struct TestStruct {
        value: i32,
    }

    impl std::fmt::Debug for TestStruct {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.debug_list().entry(self.value).finish()
        }
    }

    let test_item = TestStruct { value: 42 };
    let result = format!("{:?}", test_item);
    assert_eq!(result, "[42]");
}

#[test]
fn test_fmt_multiple_entries() {
    #[derive(Clone)]
    struct TestStruct {
        values: Vec<i32>,
    }

    impl std::fmt::Debug for TestStruct {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.debug_list().entries(self.values.iter().cloned()).finish()
        }
    }

    let test_item = TestStruct { values: vec![1, 2, 3] };
    let result = format!("{:?}", test_item);
    assert_eq!(result, "[1, 2, 3]");
}

#[test]
#[should_panic]
fn test_fmt_panic_on_clone() {
    #[derive(Clone)]
    struct NonClone;

    impl std::fmt::Debug for NonClone {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str("NonClone")
        }
    }

    let test_item: NonClone = NonClone;
    // Trying to clone before formatting
    let _ = format!("{:?}", test_item);
}

