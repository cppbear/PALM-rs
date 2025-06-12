// Answer 0

#[test]
fn test_serialize_element_with_invalid_value() {
    struct ValidOk;
    struct ValidError;

    impl crate::ser::Error for ValidError {}

    let mut impossible: crate::ser::Impossible<ValidOk, ValidError> = crate::ser::Impossible {
        void: std::mem::MaybeUninit::uninit().assume_init(),
        ok: std::marker::PhantomData,
        error: std::marker::PhantomData,
    };

    let result: Result<(), ValidError> = impossible.serialize_element(&());
    assert!(result.is_err());
}

#[test]
#[should_panic]
fn test_serialize_element_with_panic() {
    struct ValidOk;
    struct ValidError;

    impl crate::ser::Error for ValidError {}

    let mut impossible: crate::ser::Impossible<ValidOk, ValidError> = crate::ser::Impossible {
        void: std::mem::MaybeUninit::uninit().assume_init(),
        ok: std::marker::PhantomData,
        error: std::marker::PhantomData,
    };

    impossible.serialize_element(&(42));
}

#[test]
fn test_serialize_element_with_no_operation() {
    struct ValidOk;
    struct ValidError;

    impl crate::ser::Error for ValidError {}

    let mut impossible: crate::ser::Impossible<ValidOk, ValidError> = crate::ser::Impossible {
        void: std::mem::MaybeUninit::uninit().assume_init(),
        ok: std::marker::PhantomData,
        error: std::marker::PhantomData,
    };

    let result: Result<(), ValidError> = impossible.serialize_element(&"test");
    assert!(result.is_err());
}

