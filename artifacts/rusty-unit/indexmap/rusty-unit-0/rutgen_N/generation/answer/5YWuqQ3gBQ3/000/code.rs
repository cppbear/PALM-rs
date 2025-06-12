// Answer 0

#[test]
fn test_fmt_with_non_empty_structure() {
    use std::fmt;

    #[derive(Clone)]
    struct MockStruct {
        data: Vec<i32>,
    }

    impl MockStruct {
        fn new(data: Vec<i32>) -> Self {
            Self { data }
        }
    }

    impl fmt::Debug for MockStruct {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_list().entries(self.data.iter()).finish()
        }
    }

    let mock = MockStruct::new(vec![1, 2, 3]);
    let mut output = String::new();
    write!(&mut output, "{:?}", mock).unwrap();

    assert_eq!(output, "[1, 2, 3]");
}

#[test]
fn test_fmt_with_empty_structure() {
    use std::fmt;

    #[derive(Clone)]
    struct MockStruct {
        data: Vec<i32>,
    }

    impl MockStruct {
        fn new(data: Vec<i32>) -> Self {
            Self { data }
        }
    }

    impl fmt::Debug for MockStruct {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_list().entries(self.data.iter()).finish()
        }
    }

    let mock = MockStruct::new(vec![]);
    let mut output = String::new();
    write!(&mut output, "{:?}", mock).unwrap();

    assert_eq!(output, "[]");
}

