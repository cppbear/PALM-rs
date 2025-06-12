// Answer 0

#[test]
fn test_fmt_empty() {
    struct DummyStruct {
        inner: Vec<i32>,
    }
    
    impl DummyStruct {
        fn new() -> Self {
            DummyStruct { inner: Vec::new() }
        }
    }

    let dummy = DummyStruct::new();
    let mut output = String::new();
    let result = write!(&mut output, "{:?}", dummy);
    assert!(result.is_ok());
    assert_eq!(output, "[]");
}

#[test]
fn test_fmt_single_element() {
    struct DummyStruct {
        inner: Vec<i32>,
    }
    
    impl DummyStruct {
        fn new(inner: Vec<i32>) -> Self {
            DummyStruct { inner }
        }
    }

    let dummy = DummyStruct::new(vec![42]);
    let mut output = String::new();
    let result = write!(&mut output, "{:?}", dummy);
    assert!(result.is_ok());
    assert_eq!(output, "[42]");
}

#[test]
fn test_fmt_multiple_elements() {
    struct DummyStruct {
        inner: Vec<i32>,
    }
    
    impl DummyStruct {
        fn new(inner: Vec<i32>) -> Self {
            DummyStruct { inner }
        }
    }

    let dummy = DummyStruct::new(vec![1, 2, 3, 4]);
    let mut output = String::new();
    let result = write!(&mut output, "{:?}", dummy);
    assert!(result.is_ok());
    assert_eq!(output, "[1, 2, 3, 4]");
}

#[test]
#[should_panic]
fn test_fmt_panic_on_invalid_state() {
    struct DummyStruct {
        inner: Vec<i32>,
    }
    
    impl DummyStruct {
        fn new(inner: Vec<i32>) -> Self {
            DummyStruct { inner }
        }
    }

    let dummy = DummyStruct::new(vec![1, 2, 3]);
    
    // Create a context that simulates a panic
    panic!("Simulating panic in fmt test");
}

