// Answer 0

#[test]
fn test_impossible_end() {
    struct TestOk;
    struct TestError;

    impl ser::Error for TestError {}

    let impossible: Impossible<TestOk, TestError> = Impossible {
        void: std::mem::MaybeUninit::uninit().assume_init(),
        ok: std::marker::PhantomData,
        error: std::marker::PhantomData,
    };

    let result = impossible.end();
    match result {
        Err(_) => panic!("Expected Ok but got Err"),
        _ => {} // Expected panic here
    }
}

#[should_panic]
#[test]
fn test_impossible_end_should_panic() {
    struct TestOk;
    struct TestError;

    impl ser::Error for TestError {}

    let impossible: Impossible<TestOk, TestError> = Impossible {
        void: std::mem::MaybeUninit::uninit().assume_init(),
        ok: std::marker::PhantomData,
        error: std::marker::PhantomData,
    };

    let _ = impossible.end();
}

