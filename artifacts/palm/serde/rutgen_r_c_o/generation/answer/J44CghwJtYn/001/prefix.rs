// Answer 0

#[test]
fn test_end_with_void() {
    struct DummyError;
    impl ser::Error for DummyError {}

    let impossible: Impossible<(), DummyError> = Impossible {
        void: std::mem::MaybeUninit::uninit().assume_init(),
        ok: PhantomData,
        error: PhantomData,
    };

    let _ = impossible.end();
}

#[test]
#[should_panic]
fn test_end_with_invalid_void() {
    struct InvalidError;
    impl ser::Error for InvalidError {}

    // Create an impossible instance that cannot construct the Void type
    let impossible: Impossible<(), InvalidError> = Impossible {
        void: std::mem::MaybeUninit::uninit().assume_init(),
        ok: PhantomData,
        error: PhantomData,
    };

    // This should panic as it will try to match on an uninitialized state
    let _ = impossible.end();
}

