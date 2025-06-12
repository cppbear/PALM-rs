// Answer 0

#[test]
fn test_fmt_with_empty_set() {
    use std::fmt;

    struct MySet;

    impl MySet {
        fn clone(&self) -> Self {
            MySet
        }
    }

    impl fmt::Debug for MySet {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_list().finish()
        }
    }

    let my_set = MySet;
    let result = format!("{:?}", my_set);
    assert_eq!(result, "[]");
}

#[test]
fn test_fmt_with_single_element_set() {
    use std::fmt;

    struct MySet {
        elements: Vec<i32>,
    }

    impl MySet {
        fn new(elements: Vec<i32>) -> Self {
            MySet { elements }
        }

        fn clone(&self) -> Self {
            MySet {
                elements: self.elements.clone(),
            }
        }
    }

    impl fmt::Debug for MySet {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_list().entries(self.elements.iter()).finish()
        }
    }

    let my_set = MySet::new(vec![42]);
    let result = format!("{:?}", my_set);
    assert_eq!(result, "[42]");
}

#[test]
fn test_fmt_with_multiple_elements_set() {
    use std::fmt;

    struct MySet {
        elements: Vec<i32>,
    }

    impl MySet {
        fn new(elements: Vec<i32>) -> Self {
            MySet { elements }
        }

        fn clone(&self) -> Self {
            MySet {
                elements: self.elements.clone(),
            }
        }
    }

    impl fmt::Debug for MySet {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_list().entries(self.elements.iter()).finish()
        }
    }

    let my_set = MySet::new(vec![1, 2, 3, 4]);
    let result = format!("{:?}", my_set);
    assert_eq!(result, "[1, 2, 3, 4]");
}

