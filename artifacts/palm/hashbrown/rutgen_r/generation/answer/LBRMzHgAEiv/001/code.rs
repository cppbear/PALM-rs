// Answer 0

#[test]
fn test_fmt_with_non_empty_clone() {
    #[derive(Clone)]
    struct MyStruct {
        data: Vec<i32>,
    }

    impl fmt::Debug for MyStruct {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_list().entries(self.data.iter()).finish()
        }
    }

    let item = MyStruct { data: vec![1, 2, 3] };
    let mut output = String::new();
    let result = write!(&mut output, "{:?}", item);
    assert!(result.is_ok());
    assert_eq!(output, "[1, 2, 3]");
}

#[test]
fn test_fmt_with_empty_clone() {
    #[derive(Clone)]
    struct MyStruct {
        data: Vec<i32>,
    }

    impl fmt::Debug for MyStruct {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_list().entries(self.data.iter()).finish()
        }
    }

    let item = MyStruct { data: vec![] };
    let mut output = String::new();
    let result = write!(&mut output, "{:?}", item);
    assert!(result.is_ok());
    assert_eq!(output, "[]");
}

#[test]
#[should_panic(expected = "Panicking condition not expected.")]
fn test_fmt_with_panic_condition() {
    #[derive(Clone)]
    struct MyStruct {
        data: Vec<i32>,
    }

    impl fmt::Debug for MyStruct {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            if self.data.is_empty() {
                panic!("Panicking condition not expected.");
            }
            f.debug_list().entries(self.data.iter()).finish()
        }
    }

    let item = MyStruct { data: vec![] };
    let _ = write!(&mut String::new(), "{:?}", item);
}

