// Answer 0

#[test]
fn test_once_ref_debug_fmt() {
    struct TestType;

    let test_value = Box::into_raw(Box::new(TestType));
    let once_ref = OnceRef {
        inner: AtomicPtr::new(test_value),
        ghost: PhantomData,
    };

    let mut buffer = core::fmt::Formatter::new();
    let result = once_ref.fmt(&mut buffer);

    assert!(result.is_ok());
}

