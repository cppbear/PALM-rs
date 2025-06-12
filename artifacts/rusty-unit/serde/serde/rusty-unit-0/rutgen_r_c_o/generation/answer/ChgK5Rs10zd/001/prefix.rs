// Answer 0

#[test]
fn test_end_void_match() {
    struct ValidOk;
    struct ValidError;
    
    impl ser::Error for ValidError {}

    let impossible: Impossible<ValidOk, ValidError> = Impossible {
        void: Void,
        ok: PhantomData,
        error: PhantomData,
    };

    let _: Result<ValidOk, ValidError> = impossible.end();
}

#[test]
#[should_panic]
fn test_end_void_match_panic() {
    struct AnotherOk;
    struct AnotherError;

    impl ser::Error for AnotherError {}

    let impossible: Impossible<AnotherOk, AnotherError> = Impossible {
        void: Void,
        ok: PhantomData,
        error: PhantomData,
    };

    let _: Result<AnotherOk, AnotherError> = impossible.end(); // This will panic due to match on an empty enum
}

