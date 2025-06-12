// Answer 0

#[test]
fn test_impossible_end() {
    struct TestOk;
    struct TestError;

    let impossible: Impossible<TestOk, TestError> = Impossible {
        void: Void {},
        ok: PhantomData,
        error: PhantomData,
    };

    // This will panic because `end` tries to match on `self.void` which has no variants.
    let result = std::panic::catch_unwind(|| {
        impossible.end()
    });

    assert!(result.is_err());
}

#[test]
#[should_panic]
fn test_impossible_end_panics() {
    struct TestOk;
    struct TestError;

    let impossible: Impossible<TestOk, TestError> = Impossible {
        void: Void {},
        ok: PhantomData,
        error: PhantomData,
    };

    impossible.end();
}

