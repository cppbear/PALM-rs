// Answer 0

#[test]
fn test_end_impossible() {
    struct TestOk;
    struct TestError;

    impl ser::Error for TestError {}

    let impossible: Impossible<TestOk, TestError> = Impossible {
        void: std::mem::MaybeUninit::uninit().assume_init(),
        ok: PhantomData,
        error: PhantomData,
    };

    let result: Result<TestOk, TestError> = impossible.end();
    // The result should be a panic, as `end` matches on `self.void` which is of type `Void`.
    assert!(result.is_err());
}

