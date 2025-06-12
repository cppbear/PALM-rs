// Answer 0

#[test]
fn test_ident_fragment_span_none() {
    struct TestStruct;

    impl IdentFragment for TestStruct {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "TestStruct")
        }
    }

    let test_instance = TestStruct;
    assert_eq!(test_instance.span(), None);
}

#[test]
fn test_reference_ident_fragment_span_none() {
    struct TestStruct;

    impl IdentFragment for TestStruct {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "TestStruct")
        }
    }

    let test_instance = TestStruct;
    let test_reference = &test_instance;
    assert_eq!(test_reference.span(), None);
}

#[test]
fn test_mut_reference_ident_fragment_span_none() {
    struct TestStruct;

    impl IdentFragment for TestStruct {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "TestStruct")
        }
    }

    let mut test_instance = TestStruct;
    let test_mut_reference = &mut test_instance;
    assert_eq!(test_mut_reference.span(), None);
}

