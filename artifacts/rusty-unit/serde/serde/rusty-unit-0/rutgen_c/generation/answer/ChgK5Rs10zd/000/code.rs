// Answer 0

#[test]
fn test_impossible_end() {
    struct TestOk;
    struct TestError;

    impl serde::ser::Error for TestError {}

    let impossible_instance: Impossible<TestOk, TestError> = Impossible {
        void: Void {},
        ok: PhantomData,
        error: PhantomData,
    };

    let result: Result<TestOk, TestError> = impossible_instance.end();
    match result {
        Ok(_) => panic!("Expected an error, but got Ok"),
        Err(_) => {}, // This is expected due to the match on `self.void`
    }
}

#[should_panic]
fn test_impossible_end_panic() {
    struct TestOk;
    struct TestError;

    impl serde::ser::Error for TestError {}

    let impossible_instance: Impossible<TestOk, TestError> = Impossible {
        void: Void {},
        ok: PhantomData,
        error: PhantomData,
    };

    let _ = impossible_instance.end(); // This should panic
}

