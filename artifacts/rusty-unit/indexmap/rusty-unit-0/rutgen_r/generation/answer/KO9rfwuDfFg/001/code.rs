// Answer 0

#[test]
fn test_fmt_with_non_empty_iter() {
    struct Bucket {
        refs: i32,
    }

    struct TestStruct {
        iter: Vec<Bucket>,
    }

    impl TestStruct {
        fn new() -> Self {
            TestStruct { iter: vec![Bucket { refs: 1 }, Bucket { refs: 2 }, Bucket { refs: 3 }] }
        }

        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            let iter = self.iter.as_slice().iter().map(|bucket| bucket.refs);
            f.debug_list().entries(iter).finish()
        }
    }

    let test_struct = TestStruct::new();
    let result = format!("{:?}", test_struct);
    assert_eq!(result, "[1, 2, 3]");
}

#[test]
fn test_fmt_with_empty_iter() {
    struct Bucket {
        refs: i32,
    }

    struct TestStruct {
        iter: Vec<Bucket>,
    }

    impl TestStruct {
        fn new() -> Self {
            TestStruct { iter: vec![] }
        }

        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            let iter = self.iter.as_slice().iter().map(|bucket| bucket.refs);
            f.debug_list().entries(iter).finish()
        }
    }

    let test_struct = TestStruct::new();
    let result = format!("{:?}", test_struct);
    assert_eq!(result, "[]");
}

#[test]
#[should_panic]
fn test_fmt_with_panic_condition() {
    struct Bucket {
        refs: i32,
    }

    struct TestStruct {
        iter: Vec<Bucket>,
    }

    impl TestStruct {
        fn new() -> Self {
            TestStruct { iter: vec![Bucket { refs: 1 }, Bucket { refs: 2 }] }
        }

        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            let iter = self.iter.as_slice().iter().map(|bucket| bucket.refs);
            // Force panic for demonstrative purposes
            if self.iter.len() < 3 {
                panic!("Intentional panic for test");
            }
            f.debug_list().entries(iter).finish()
        }
    }

    let test_struct = TestStruct::new();
    let _ = test_struct.fmt(&mut std::fmt::Formatter::new());
}

