// Answer 0

#[test]
fn test_fmt_with_non_empty_iterator() {
    struct MyIter {
        iter: Vec<i32>,
    }

    impl MyIter {
        fn new() -> Self {
            MyIter { iter: vec![1, 2, 3] }
        }
    }

    let my_iter = MyIter::new();
    let mut buffer = String::new();
    let result = write!(&mut buffer, "{:?}", my_iter.iter);
    assert!(result.is_ok());
    assert_eq!(buffer, "[1, 2, 3]");
}

#[test]
fn test_fmt_with_empty_iterator() {
    struct MyIter {
        iter: Vec<i32>,
    }

    impl MyIter {
        fn new() -> Self {
            MyIter { iter: vec![] }
        }
    }

    let my_iter = MyIter::new();
    let mut buffer = String::new();
    let result = write!(&mut buffer, "{:?}", my_iter.iter);
    assert!(result.is_ok());
    assert_eq!(buffer, "[]");
}

#[test]
fn test_fmt_with_single_element_iterator() {
    struct MyIter {
        iter: Vec<i32>,
    }

    impl MyIter {
        fn new() -> Self {
            MyIter { iter: vec![42] }
        }
    }

    let my_iter = MyIter::new();
    let mut buffer = String::new();
    let result = write!(&mut buffer, "{:?}", my_iter.iter);
    assert!(result.is_ok());
    assert_eq!(buffer, "[42]");
}

#[test]
#[should_panic]
fn test_fmt_with_panic() {
    struct MyIter {
        iter: Vec<i32>,
    }

    impl MyIter {
        fn new() -> Self {
            MyIter { iter: vec![1, 2, 3] }
        }
    }

    let my_iter = MyIter::new();
    let _ = write!("This will panic: {:?}", my_iter.iter.get(100).unwrap());
}

