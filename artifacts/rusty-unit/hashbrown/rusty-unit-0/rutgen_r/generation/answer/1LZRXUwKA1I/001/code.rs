// Answer 0

#[test]
fn test_fmt_empty_set() {
    struct TestSet {
        elements: Vec<i32>,
    }

    impl TestSet {
        fn new() -> Self {
            TestSet { elements: Vec::new() }
        }
    }

    impl Clone for TestSet {
        fn clone(&self) -> Self {
            TestSet { elements: self.elements.clone() }
        }
    }

    use std::fmt;

    impl fmt::Debug for TestSet {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_list().entries(&self.elements).finish()
        }
    }

    let empty_set = TestSet::new();
    let output = format!("{:?}", empty_set);
    assert_eq!(output, "[]");
}

#[test]
fn test_fmt_single_element_set() {
    struct TestSet {
        elements: Vec<i32>,
    }

    impl TestSet {
        fn new(elements: Vec<i32>) -> Self {
            TestSet { elements }
        }
    }

    impl Clone for TestSet {
        fn clone(&self) -> Self {
            TestSet { elements: self.elements.clone() }
        }
    }

    use std::fmt;

    impl fmt::Debug for TestSet {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_list().entries(&self.elements).finish()
        }
    }

    let single_element_set = TestSet::new(vec![1]);
    let output = format!("{:?}", single_element_set);
    assert_eq!(output, "[1]");
}

#[test]
fn test_fmt_multiple_element_set() {
    struct TestSet {
        elements: Vec<i32>,
    }

    impl TestSet {
        fn new(elements: Vec<i32>) -> Self {
            TestSet { elements }
        }
    }

    impl Clone for TestSet {
        fn clone(&self) -> Self {
            TestSet { elements: self.elements.clone() }
        }
    }

    use std::fmt;

    impl fmt::Debug for TestSet {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_list().entries(&self.elements).finish()
        }
    }

    let multiple_element_set = TestSet::new(vec![1, 2, 3]);
    let output = format!("{:?}", multiple_element_set);
    assert_eq!(output, "[1, 2, 3]");
}

