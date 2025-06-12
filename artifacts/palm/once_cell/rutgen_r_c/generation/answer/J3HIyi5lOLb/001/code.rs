// Answer 0

#[test]
fn test_once_ref_debug_fmt_valid() {
    struct TestStruct {
        value: i32,
    }

    let value = Box::into_raw(Box::new(TestStruct { value: 42 }));
    let once_ref = OnceRef {
        inner: AtomicPtr::new(value),
        ghost: PhantomData,
    };

    let mut output = core::fmt::Formatter::new();
    
    let result = once_ref.fmt(&mut output);
    
    assert!(result.is_ok());
    assert_eq!(format!("{:?}", once_ref), format!("OnceRef({:?})", once_ref.inner));
}

#[test]
#[should_panic]
fn test_once_ref_debug_fmt_null_pointer() {
    let once_ref = OnceRef {
        inner: AtomicPtr::new(ptr::null_mut()),
        ghost: PhantomData,
    };

    let mut output = core::fmt::Formatter::new();
    
    let _ = once_ref.fmt(&mut output); // This should panic due to null pointer dereference.
}

