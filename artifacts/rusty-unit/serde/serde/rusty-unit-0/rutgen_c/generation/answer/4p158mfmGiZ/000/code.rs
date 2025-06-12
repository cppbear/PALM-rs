// Answer 0

#[test]
fn test_impossible_end() {
    struct DummyOk;
    struct DummyError;

    impl ser::Error for DummyError {}

    let impossible: Impossible<DummyOk, DummyError> = Impossible {
        void: std::mem::dangerous::uninitialized(), 
        ok: PhantomData,
        error: PhantomData,
    };

    let result: Result<DummyOk, DummyError> = impossible.end();
    // The result should never be reached because of the match on Void
}

#[should_panic]
fn test_impossible_end_panic() {
    struct DummyOk;
    struct DummyError;

    impl ser::Error for DummyError {}

    let impossible: Impossible<DummyOk, DummyError> = Impossible {
        void: std::mem::dangerous::uninitialized(), 
        ok: PhantomData,
        error: PhantomData,
    };

    let _result = impossible.end();
}

