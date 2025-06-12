// Answer 0

#[test]
fn test_end_with_valid_types() {
    struct ValidOk;
    struct ValidError;

    let instance: Impossible<ValidOk, ValidError> = Impossible {
        void: std::mem::MaybeUninit::uninit().assume_init(),
        ok: PhantomData,
        error: PhantomData,
    };

    let result = instance.end();
}

#[test]
#[should_panic]
fn test_end_with_void_type() {
    struct VoidOk;
    struct VoidError;

    let instance: Impossible<VoidOk, VoidError> = Impossible {
        void: std::mem::MaybeUninit::uninit().assume_init(),
        ok: PhantomData,
        error: PhantomData,
    };

    let result = instance.end();
}

#[test]
fn test_end_with_other_types() {
    struct AnotherOk;
    struct AnotherError;

    let instance: Impossible<AnotherOk, AnotherError> = Impossible {
        void: std::mem::MaybeUninit::uninit().assume_init(),
        ok: PhantomData,
        error: PhantomData,
    };

    let result = instance.end();
}

