// Answer 0

#[test]
fn test_fmt_empty() {
    struct TestStruct {
        data: Vec<i32>,
    }

    impl Clone for TestStruct {
        fn clone(&self) -> Self {
            TestStruct { data: self.data.clone() }
        }
    }

    impl std::fmt::Debug for TestStruct {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.debug_list().entries(self.data.iter()).finish()
        }
    }

    let empty_struct = TestStruct { data: vec![] };
    let result = format!("{:?}", empty_struct);
    assert_eq!(result, "[]");
}

#[test]
fn test_fmt_non_empty() {
    struct TestStruct {
        data: Vec<i32>,
    }

    impl Clone for TestStruct {
        fn clone(&self) -> Self {
            TestStruct { data: self.data.clone() }
        }
    }

    impl std::fmt::Debug for TestStruct {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.debug_list().entries(self.data.iter()).finish()
        }
    }

    let non_empty_struct = TestStruct { data: vec![1, 2, 3] };
    let result = format!("{:?}", non_empty_struct);
    assert_eq!(result, "[1, 2, 3]");
}

#[test]
fn test_fmt_large_data() {
    struct TestStruct {
        data: Vec<i32>,
    }

    impl Clone for TestStruct {
        fn clone(&self) -> Self {
            TestStruct { data: self.data.clone() }
        }
    }

    impl std::fmt::Debug for TestStruct {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.debug_list().entries(self.data.iter()).finish()
        }
    }

    let large_struct = TestStruct { data: (0..1000).collect() };
    let result = format!("{:?}", large_struct);
    let expected: String = (0..1000).map(|x| x.to_string()).collect::<Vec<String>>().join(", ");
    assert_eq!(result, format!("[{}]", expected));
}

#[should_panic]
#[test]
fn test_fmt_panic_on_clone() {
    struct PanicStruct;

    impl Clone for PanicStruct {
        fn clone(&self) -> Self {
            panic!("Clone called")
        }
    }

    impl std::fmt::Debug for PanicStruct {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.debug_list().entries(vec![1, 2, 3]).finish()
        }
    }

    let panic_struct = PanicStruct;
    let _ = format!("{:?}", panic_struct);
}

