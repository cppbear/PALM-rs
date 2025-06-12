// Answer 0

#[test]
fn test_once_box_debug_fmt() {
    use super::OnceBox;
    use core::atomic::AtomicPtr;
    use core::ptr;
    use alloc::boxed::Box;

    struct TestStruct {
        value: i32,
    }

    let test_value = Box::new(TestStruct { value: 42 });
    let atomic_ptr = AtomicPtr::new(Box::into_raw(test_value));
    
    let once_box = OnceBox {
        inner: atomic_ptr,
        ghost: PhantomData,
    };

    let mut output = core::fmt::Formatter::new();
    let result = once_box.fmt(&mut output);
    
    assert!(result.is_ok());
}

