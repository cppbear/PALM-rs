// Answer 0

#[test]
fn test_fmt_empty() {
    struct TestData {
        inner: Vec<(i32, i32)>
    }

    impl TestData {
        fn new() -> Self {
            TestData { inner: Vec::new() }
        }
    }

    let data = TestData::new();
    let formatted = format!("{:?}", data);
    assert_eq!(formatted, "[]"); // Expecting an empty list output
}

#[test]
fn test_fmt_single_element() {
    struct TestData {
        inner: Vec<(i32, i32)>
    }

    impl TestData {
        fn new() -> Self {
            TestData { inner: vec![(1, 100)] }
        }
    }

    let data = TestData::new();
    let formatted = format!("{:?}", data);
    assert_eq!(formatted, "[100]"); // Expecting the single value output
}

#[test]
fn test_fmt_multiple_elements() {
    struct TestData {
        inner: Vec<(i32, i32)>
    }

    impl TestData {
        fn new() -> Self {
            TestData { inner: vec![(1, 100), (2, 200), (3, 300)] }
        }
    }

    let data = TestData::new();
    let formatted = format!("{:?}", data);
    assert_eq!(formatted, "[100, 200, 300]"); // Expecting all values output
}

#[test]
fn test_fmt_with_duplicates() {
    struct TestData {
        inner: Vec<(i32, i32)>
    }

    impl TestData {
        fn new() -> Self {
            TestData { inner: vec![(1, 100), (2, 100), (3, 300)] }
        }
    }

    let data = TestData::new();
    let formatted = format!("{:?}", data);
    assert_eq!(formatted, "[100, 100, 300]"); // Expecting duplicates included
}

#[test]
#[should_panic]
fn test_fmt_with_panic() {
    struct TestData {
        inner: Vec<(i32, i32)>
    }

    impl TestData {
        fn new() -> Self {
            TestData { inner: vec![(1, 100), (2, 200)] }
        }
    }
    
    let data = TestData::new();
    // Simulate a panic scenario by calling a method that modifies state inappropriately (hypothetical)
    panic!("Intentional panic in test");
}

