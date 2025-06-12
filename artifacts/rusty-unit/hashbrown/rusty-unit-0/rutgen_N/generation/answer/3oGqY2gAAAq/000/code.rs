// Answer 0

#[test]
fn test_fmt_empty_set() {
    use std::fmt;

    struct EmptySet;

    impl fmt::Debug for EmptySet {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_list().finish()
        }
    }

    let empty_set = EmptySet;
    let mut output = Vec::new();
    let result = write!(&mut output, "{:?}", empty_set);

    assert!(result.is_ok());
    assert_eq!(String::from_utf8(output).unwrap(), "[]");
}

#[test]
fn test_fmt_single_element_set() {
    use std::fmt;

    struct SingleElementSet {
        element: i32,
    }

    impl fmt::Debug for SingleElementSet {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_list().entry(&self.element).finish()
        }
    }

    let single_set = SingleElementSet { element: 42 };
    let mut output = Vec::new();
    let result = write!(&mut output, "{:?}", single_set);

    assert!(result.is_ok());
    assert_eq!(String::from_utf8(output).unwrap(), "[42]");
}

#[test]
fn test_fmt_multiple_elements_set() {
    use std::fmt;

    struct MultipleElementsSet {
        elements: Vec<i32>,
    }

    impl fmt::Debug for MultipleElementsSet {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let mut debug_list = f.debug_list();
            for element in &self.elements {
                debug_list.entry(element);
            }
            debug_list.finish()
        }
    }

    let multi_set = MultipleElementsSet { elements: vec![1, 2, 3] };
    let mut output = Vec::new();
    let result = write!(&mut output, "{:?}", multi_set);

    assert!(result.is_ok());
    assert_eq!(String::from_utf8(output).unwrap(), "[1, 2, 3]");
}

