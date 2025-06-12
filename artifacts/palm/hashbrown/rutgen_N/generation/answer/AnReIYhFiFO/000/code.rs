// Answer 0

#[test]
fn test_fmt_empty() {
    struct TestStruct {
        data: Vec<i32>,
    }

    impl TestStruct {
        fn iter(&self) -> std::slice::Iter<i32> {
            self.data.iter()
        }
    }

    let test_instance = TestStruct { data: Vec::new() };
    let mut output = String::new();
    let result = write!(&mut output, "{:?}", test_instance);
    assert!(result.is_ok());
    assert_eq!(output, "[]");
}

#[test]
fn test_fmt_single_element() {
    struct TestStruct {
        data: Vec<i32>,
    }

    impl TestStruct {
        fn iter(&self) -> std::slice::Iter<i32> {
            self.data.iter()
        }
    }

    let test_instance = TestStruct { data: vec![42] };
    let mut output = String::new();
    let result = write!(&mut output, "{:?}", test_instance);
    assert!(result.is_ok());
    assert_eq!(output, "[42]");
}

#[test]
fn test_fmt_multiple_elements() {
    struct TestStruct {
        data: Vec<i32>,
    }

    impl TestStruct {
        fn iter(&self) -> std::slice::Iter<i32> {
            self.data.iter()
        }
    }

    let test_instance = TestStruct { data: vec![1, 2, 3] };
    let mut output = String::new();
    let result = write!(&mut output, "{:?}", test_instance);
    assert!(result.is_ok());
    assert_eq!(output, "[1, 2, 3]");
}

