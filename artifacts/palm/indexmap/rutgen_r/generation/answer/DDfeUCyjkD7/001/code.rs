// Answer 0

#[test]
fn test_fmt_empty() {
    struct TestStruct;

    impl Clone for TestStruct {
        fn clone(&self) -> Self {
            TestStruct
        }
    }

    let value = TestStruct;
    let mut buffer = String::new();
    let res = std::fmt::write(&mut buffer, |f| value.fmt(f));
    assert!(res.is_ok());
    assert_eq!(buffer, "[]");
}

#[test]
fn test_fmt_single_element() {
    struct TestStruct {
        data: Vec<i32>,
    }

    impl Clone for TestStruct {
        fn clone(&self) -> Self {
            TestStruct {
                data: self.data.clone(),
            }
        }
    }

    let value = TestStruct { data: vec![1] };
    let mut buffer = String::new();
    let res = std::fmt::write(&mut buffer, |f| value.fmt(f));
    assert!(res.is_ok());
    assert_eq!(buffer, "[1]");
}

#[test]
fn test_fmt_multiple_elements() {
    struct TestStruct {
        data: Vec<i32>,
    }

    impl Clone for TestStruct {
        fn clone(&self) -> Self {
            TestStruct {
                data: self.data.clone(),
            }
        }
    }

    let value = TestStruct { data: vec![1, 2, 3] };
    let mut buffer = String::new();
    let res = std::fmt::write(&mut buffer, |f| value.fmt(f));
    assert!(res.is_ok());
    assert_eq!(buffer, "[1, 2, 3]");
}

