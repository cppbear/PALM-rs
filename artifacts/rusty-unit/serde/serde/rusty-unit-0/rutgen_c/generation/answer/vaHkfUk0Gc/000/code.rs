// Answer 0

#[test]
fn test_size_hint_both_elements_exist() {
    struct TestError;
    impl de::Error for TestError {
        fn custom<T>(_msg: T) -> Self {
            TestError
        }
    }

    let visitor = PairVisitor(Some(1), Some(2), PhantomData);
    assert_eq!(visitor.size_hint(), Some(2));
}

#[test]
fn test_size_hint_first_element_exists() {
    struct TestError;
    impl de::Error for TestError {
        fn custom<T>(_msg: T) -> Self {
            TestError
        }
    }

    let visitor = PairVisitor(Some(1), None, PhantomData);
    assert_eq!(visitor.size_hint(), Some(2));
}

#[test]
fn test_size_hint_second_element_exists() {
    struct TestError;
    impl de::Error for TestError {
        fn custom<T>(_msg: T) -> Self {
            TestError
        }
    }

    let visitor = PairVisitor(None, Some(2), PhantomData);
    assert_eq!(visitor.size_hint(), Some(1));
}

#[test]
fn test_size_hint_no_elements_exist() {
    struct TestError;
    impl de::Error for TestError {
        fn custom<T>(_msg: T) -> Self {
            TestError
        }
    }

    let visitor = PairVisitor(None, None, PhantomData);
    assert_eq!(visitor.size_hint(), Some(0));
}

