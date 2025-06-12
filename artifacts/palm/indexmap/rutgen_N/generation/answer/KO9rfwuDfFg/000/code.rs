// Answer 0

#[test]
fn test_fmt_empty() {
    struct Bucket {
        value: usize,
    }

    struct TestStruct {
        iter: Vec<Bucket>,
    }

    impl TestStruct {
        fn as_slice(&self) -> &[Bucket] {
            &self.iter
        }
    }

    let test_struct = TestStruct { iter: vec![] };
    let result = format!("{:?}", test_struct);
    assert_eq!(result, "[]");
}

#[test]
fn test_fmt_single_entry() {
    struct Bucket {
        value: usize,
    }

    struct TestStruct {
        iter: Vec<Bucket>,
    }

    impl TestStruct {
        fn as_slice(&self) -> &[Bucket] {
            &self.iter
        }
    }

    let test_struct = TestStruct { iter: vec![Bucket { value: 1 }] };
    let result = format!("{:?}", test_struct);
    assert_eq!(result, "[1]");
}

#[test]
fn test_fmt_multiple_entries() {
    struct Bucket {
        value: usize,
    }

    struct TestStruct {
        iter: Vec<Bucket>,
    }

    impl TestStruct {
        fn as_slice(&self) -> &[Bucket] {
            &self.iter
        }
    }

    let test_struct = TestStruct { iter: vec![Bucket { value: 1 }, Bucket { value: 2 }] };
    let result = format!("{:?}", test_struct);
    assert_eq!(result, "[1, 2]");
}

