// Answer 0

#[test]
fn test_fmt_with_empty_iter() {
    struct Bucket {
        refs: usize,
    }

    struct TestStruct {
        iter: Vec<Bucket>,
    }

    impl TestStruct {
        fn new() -> Self {
            TestStruct { iter: Vec::new() }
        }
    }

    let test_struct = TestStruct::new();
    let output = format!("{:?}", test_struct);
    assert_eq!(output, "[]");
}

#[test]
fn test_fmt_with_one_element() {
    struct Bucket {
        refs: usize,
    }

    struct TestStruct {
        iter: Vec<Bucket>,
    }

    impl TestStruct {
        fn new() -> Self {
            TestStruct { iter: vec![Bucket { refs: 1 }] }
        }

        fn iter(&self) -> &[Bucket] {
            &self.iter
        }
    }

    let test_struct = TestStruct::new();
    let output = format!("{:?}", test_struct);
    assert_eq!(output, "[1]");
}

#[test]
fn test_fmt_with_multiple_elements() {
    struct Bucket {
        refs: usize,
    }

    struct TestStruct {
        iter: Vec<Bucket>,
    }

    impl TestStruct {
        fn new() -> Self {
            TestStruct {
                iter: vec![Bucket { refs: 1 }, Bucket { refs: 2 }, Bucket { refs: 3 }],
            }
        }

        fn iter(&self) -> &[Bucket] {
            &self.iter
        }
    }

    let test_struct = TestStruct::new();
    let output = format!("{:?}", test_struct);
    assert_eq!(output, "[1, 2, 3]");
}

#[test]
#[should_panic]
fn test_fmt_with_panic_condition() {
    struct Bucket {
        refs: usize,
    }

    struct TestStruct {
        iter: Vec<Bucket>,
    }

    impl TestStruct {
        fn new() -> Self {
            TestStruct { iter: vec![Bucket { refs: 1 }] }
        }

        fn iter(&self) -> &[Bucket] {
            &self.iter
        }
    }

    let test_struct = TestStruct::new();
    // Intentionally cause a panic if the refs count is 0
    if test_struct.iter.first().unwrap().refs == 0 {
        panic!("Reference count is zero");
    }
}

