// Answer 0

#[test]
fn test_once_ref_inner_pointer_null() {
    let once_ref: OnceRef<i32> = OnceRef {
        inner: AtomicPtr::new(ptr::null_mut()),
        ghost: PhantomData,
    };
    let _ = core::fmt::Debug::fmt(&once_ref, &mut core::fmt::Formatter::new());
}

#[test]
fn test_once_ref_inner_pointer_valid() {
    let value: i32 = 42;
    let once_ref: OnceRef<i32> = OnceRef {
        inner: AtomicPtr::new(&value as *const i32 as *mut i32),
        ghost: PhantomData,
    };
    let _ = core::fmt::Debug::fmt(&once_ref, &mut core::fmt::Formatter::new());
}

