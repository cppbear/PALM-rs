// Answer 0

#[test]
fn test_fmt_with_empty_iterator() {
    struct TestBucket {
        refs: Vec<i32>,
    }

    struct TestStruct {
        iter: Vec<TestBucket>,
    }

    impl TestStruct {
        fn new() -> Self {
            Self { iter: vec![] }
        }
    }

    let test_struct = TestStruct::new();
    let mut output = String::new();
    let _ = write!(&mut output, "{:?}", test_struct);
    assert_eq!(output, "[]");
}

#[test]
fn test_fmt_with_single_element() {
    struct TestBucket {
        refs: Vec<i32>,
    }

    struct TestStruct {
        iter: Vec<TestBucket>,
    }

    impl TestStruct {
        fn new_with_element(element: i32) -> Self {
            Self { 
                iter: vec![TestBucket { refs: vec![element] }] 
            }
        }
    }

    let test_struct = TestStruct::new_with_element(42);
    let mut output = String::new();
    let _ = write!(&mut output, "{:?}", test_struct);
    assert_eq!(output, "[42]");
}

#[test]
fn test_fmt_with_multiple_elements() {
    struct TestBucket {
        refs: Vec<i32>,
    }

    struct TestStruct {
        iter: Vec<TestBucket>,
    }

    impl TestStruct {
        fn new_with_elements(elements: Vec<i32>) -> Self {
            Self { 
                iter: elements.into_iter().map(|e| TestBucket { refs: vec![e] }).collect() 
            }
        }
    }

    let test_struct = TestStruct::new_with_elements(vec![1, 2, 3]);
    let mut output = String::new();
    let _ = write!(&mut output, "{:?}", test_struct);
    assert_eq!(output, "[1, 2, 3]");
}

