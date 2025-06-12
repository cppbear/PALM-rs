// Answer 0

#[test]
fn test_end_with_void() {
    struct MyError;
    impl ser::Error for MyError {}

    let impossible_instance: Impossible<(), MyError> = Impossible {
        void: std::mem::zeroed(),
        ok: PhantomData,
        error: PhantomData,
    };

    let _ = impossible_instance.end();
}

#[should_panic]
fn test_end_with_void_panic() {
    struct MyError;
    impl ser::Error for MyError {}

    let impossible_instance: Impossible<u32, MyError> = Impossible {
        void: std::mem::zeroed(),
        ok: PhantomData,
        error: PhantomData,
    };

    let _ = impossible_instance.end();
}

#[test]
fn test_end_with_different_error_type() {
    struct AnotherError;
    impl ser::Error for AnotherError {}

    let impossible_instance: Impossible<bool, AnotherError> = Impossible {
        void: std::mem::zeroed(),
        ok: PhantomData,
        error: PhantomData,
    };

    let _ = impossible_instance.end();
}

#[should_panic]
fn test_end_with_void_panic_alt() {
    struct YetAnotherError;
    impl ser::Error for YetAnotherError {}

    let impossible_instance: Impossible<String, YetAnotherError> = Impossible {
        void: std::mem::zeroed(),
        ok: PhantomData,
        error: PhantomData,
    };

    let _ = impossible_instance.end();
}

