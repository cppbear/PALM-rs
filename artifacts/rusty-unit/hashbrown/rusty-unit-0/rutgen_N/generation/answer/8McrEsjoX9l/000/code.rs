// Answer 0

#[test]
fn test_fmt_empty() {
    struct TestStruct {
        inner: std::collections::HashMap<i32, i32>,
    }

    impl std::fmt::Debug for TestStruct {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.debug_list()
                .entries(self.inner.iter().map(|(_, val)| val))
                .finish()
        }
    }

    let test_struct = TestStruct {
        inner: std::collections::HashMap::new(),
    };

    let mut output = String::new();
    {
        let formatter = &mut std::fmt::Formatter::new(&mut output);
        test_struct.fmt(formatter).unwrap();
    }

    assert_eq!(output, "[]");
}

#[test]
fn test_fmt_single_entry() {
    struct TestStruct {
        inner: std::collections::HashMap<i32, i32>,
    }

    impl std::fmt::Debug for TestStruct {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.debug_list()
                .entries(self.inner.iter().map(|(_, val)| val))
                .finish()
        }
    }

    let mut inner = std::collections::HashMap::new();
    inner.insert(1, 10);

    let test_struct = TestStruct { inner };

    let mut output = String::new();
    {
        let formatter = &mut std::fmt::Formatter::new(&mut output);
        test_struct.fmt(formatter).unwrap();
    }

    assert_eq!(output, "[10]");
}

#[test]
fn test_fmt_multiple_entries() {
    struct TestStruct {
        inner: std::collections::HashMap<i32, i32>,
    }

    impl std::fmt::Debug for TestStruct {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.debug_list()
                .entries(self.inner.iter().map(|(_, val)| val))
                .finish()
        }
    }

    let mut inner = std::collections::HashMap::new();
    inner.insert(1, 10);
    inner.insert(2, 20);
    inner.insert(3, 30);

    let test_struct = TestStruct { inner };

    let mut output = String::new();
    {
        let formatter = &mut std::fmt::Formatter::new(&mut output);
        test_struct.fmt(formatter).unwrap();
    }

    assert_eq!(output, "[10, 20, 30]");
}

