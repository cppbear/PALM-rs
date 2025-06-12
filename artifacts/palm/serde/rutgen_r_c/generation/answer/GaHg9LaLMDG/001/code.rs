// Answer 0

#[test]
fn test_unit_variant_success() {
    struct TestError;

    impl de::Error for TestError {
        fn custom<T>(_: T) -> Self {
            TestError
        }
    }

    let variant_access = UnitOnly::<TestError> {
        marker: PhantomData,
    };

    let result: Result<(), TestError> = variant_access.unit_variant();
    assert!(result.is_ok());
    assert_eq!(result, Ok(()));
}

