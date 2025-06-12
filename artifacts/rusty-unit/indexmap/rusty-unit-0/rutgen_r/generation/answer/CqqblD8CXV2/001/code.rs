// Answer 0

#[test]
fn test_fmt_empty_slice() {
    struct TestSlice;

    impl core::fmt::Debug for TestSlice {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            f.debug_list().finish()
        }
    }

    let test_slice = TestSlice;
    let result = format!("{:?}", test_slice);
    assert_eq!(result, "[]");
}

#[test]
fn test_fmt_single_element_slice() {
    struct TestSlice {
        elements: Vec<i32>,
    }

    impl core::fmt::Debug for TestSlice {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            f.debug_list().entries(&self.elements).finish()
        }
    }

    let test_slice = TestSlice { elements: vec![42] };
    let result = format!("{:?}", test_slice);
    assert_eq!(result, "[42]");
}

#[test]
fn test_fmt_multiple_element_slice() {
    struct TestSlice {
        elements: Vec<i32>,
    }

    impl core::fmt::Debug for TestSlice {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            f.debug_list().entries(&self.elements).finish()
        }
    }

    let test_slice = TestSlice { elements: vec![1, 2, 3] };
    let result = format!("{:?}", test_slice);
    assert_eq!(result, "[1, 2, 3]");
}

#[test]
#[should_panic]
fn test_fmt_panic_on_invalid_format() {
    struct FaultySlice;

    impl core::fmt::Debug for FaultySlice {
        fn fmt(&self, _: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            panic!("Forced panic for testing");
        }
    }

    let faulty_slice = FaultySlice;
    let _ = format!("{:?}", faulty_slice);
}

