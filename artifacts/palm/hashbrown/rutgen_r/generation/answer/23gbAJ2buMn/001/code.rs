// Answer 0

#[test]
fn test_fmt_empty() {
    struct TestStruct {
        inner: Vec<i32>,
    }

    let test = TestStruct { inner: Vec::new() };
    let result = format!("{:?}", test);
    assert_eq!(result, "[]");
}

#[test]
fn test_fmt_single_element() {
    struct TestStruct {
        inner: Vec<i32>,
    }

    let test = TestStruct { inner: vec![42] };
    let result = format!("{:?}", test);
    assert_eq!(result, "[42]");
}

#[test]
fn test_fmt_multiple_elements() {
    struct TestStruct {
        inner: Vec<i32>,
    }

    let test = TestStruct { inner: vec![1, 2, 3, 4, 5] };
    let result = format!("{:?}", test);
    assert_eq!(result, "[1, 2, 3, 4, 5]");
}

#[test]
fn test_fmt_clone_panic() {
    use std::fmt;

    struct PanickingStruct {
        inner: Vec<i32>,
    }

    impl fmt::Debug for PanickingStruct {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            panic!("This is a panic for testing purposes.");
        }
    }

    let test = PanickingStruct { inner: vec![1, 2, 3] };
    let result = std::panic::catch_unwind(|| format!("{:?}", test));
    assert!(result.is_err());
}

