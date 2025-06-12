// Answer 0

#[test]
fn test_serialize_element_void() {
    struct TestError;
    impl ser::Error for TestError {}
    
    let mut instance: Impossible<(), TestError> = Impossible { 
        void: std::mem::unreachable(), 
        ok: std::marker::PhantomData, 
        error: std::marker::PhantomData 
    };

    let result: Result<(), TestError> = instance.serialize_element(&());

    // We expect this to panic due to reaching the unreachable pattern when accessing `self.void`.
    let panic_result = std::panic::catch_unwind(|| {
        instance.serialize_element(&());
    });

    assert!(panic_result.is_err());
}

#[test]
fn test_serialize_element_no_value() {
    struct TestError;
    impl ser::Error for TestError {}

    let mut instance: Impossible<(), TestError> = Impossible {
        void: std::mem::unreachable(),
        ok: std::marker::PhantomData,
        error: std::marker::PhantomData
    };

    let panic_result = std::panic::catch_unwind(|| {
        instance.serialize_element::<()>(std::ptr::null());
    });

    assert!(panic_result.is_err());
}

