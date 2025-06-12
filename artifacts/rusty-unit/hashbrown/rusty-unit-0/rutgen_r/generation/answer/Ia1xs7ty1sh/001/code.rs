// Answer 0

#[test]
fn test_fmt_with_empty_clone() {
    #[derive(Clone)]
    struct TestData {
        data: Vec<i32>,
    }

    impl std::fmt::Debug for TestData {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.debug_list().entries(self.data.iter()).finish()
        }
    }

    let empty_data = TestData { data: vec![] };
    let mut output = String::new();
    {
        let formatter = &mut std::fmt::Formatter::new();
        empty_data.fmt(formatter).unwrap();
        output = formatter.to_string();
    }
    assert_eq!(output, "[]");
}

#[test]
fn test_fmt_with_single_element() {
    #[derive(Clone)]
    struct TestData {
        data: Vec<i32>,
    }

    impl std::fmt::Debug for TestData {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.debug_list().entries(self.data.iter()).finish()
        }
    }

    let single_data = TestData { data: vec![42] };
    let mut output = String::new();
    {
        let formatter = &mut std::fmt::Formatter::new();
        single_data.fmt(formatter).unwrap();
        output = formatter.to_string();
    }
    assert_eq!(output, "[42]");
}

#[test]
fn test_fmt_with_multiple_elements() {
    #[derive(Clone)]
    struct TestData {
        data: Vec<i32>,
    }

    impl std::fmt::Debug for TestData {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.debug_list().entries(self.data.iter()).finish()
        }
    }

    let multiple_data = TestData { data: vec![1, 2, 3] };
    let mut output = String::new();
    {
        let formatter = &mut std::fmt::Formatter::new();
        multiple_data.fmt(formatter).unwrap();
        output = formatter.to_string();
    }
    assert_eq!(output, "[1, 2, 3]");
}

#[test]
#[should_panic]
fn test_fmt_with_invalid_state() {
    #[derive(Clone)]
    struct TestData {
        data: Vec<i32>,
    }

    impl std::fmt::Debug for TestData {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            panic!("Intentional panic for testing");
        }
    }

    let panic_data = TestData { data: vec![1] };
    let _ = panic_data.fmt(&mut std::fmt::Formatter::new());
}

