// Answer 0

#[test]
fn test_fmt_with_empty_iter() {
    struct EmptyIter;

    impl Iterator for EmptyIter {
        type Item = ();

        fn next(&mut self) -> Option<Self::Item> {
            None
        }
    }

    struct TestStruct;

    impl TestStruct {
        fn iter(&self) -> EmptyIter {
            EmptyIter
        }
    }

    let test_struct = TestStruct;
    let mut output = String::new();
    let result = write!(&mut output, "{:?}", test_struct);
    assert!(result.is_ok());
    assert_eq!(output, "[]");
}

#[test]
fn test_fmt_with_single_element_iter() {
    struct SingleElementIter;

    impl Iterator for SingleElementIter {
        type Item = i32;

        fn next(&mut self) -> Option<Self::Item> {
            Some(42)
        }
    }

    struct TestStruct;

    impl TestStruct {
        fn iter(&self) -> SingleElementIter {
            SingleElementIter
        }
    }

    let test_struct = TestStruct;
    let mut output = String::new();
    let result = write!(&mut output, "{:?}", test_struct);
    assert!(result.is_ok());
    assert_eq!(output, "[42]");
}

#[test]
fn test_fmt_with_multiple_elements_iter() {
    struct MultipleElementsIter {
        count: usize,
    }

    impl Iterator for MultipleElementsIter {
        type Item = i32;

        fn next(&mut self) -> Option<Self::Item> {
            if self.count < 3 {
                self.count += 1;
                Some(self.count as i32 * 10)
            } else {
                None
            }
        }
    }

    struct TestStruct;

    impl TestStruct {
        fn iter(&self) -> MultipleElementsIter {
            MultipleElementsIter { count: 0 }
        }
    }

    let test_struct = TestStruct;
    let mut output = String::new();
    let result = write!(&mut output, "{:?}", test_struct);
    assert!(result.is_ok());
    assert_eq!(output, "[10, 20, 30]");
}

