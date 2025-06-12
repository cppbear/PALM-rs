// Answer 0

#[test]
fn test_once_box_fmt_with_non_null_pointer() {
    use super::OnceBox;
    use core::ptr;
    
    struct TestData;
    
    let test_value = Box::new(TestData);
    let test_ptr = Box::into_raw(test_value);
    
    let once_box = OnceBox {
        inner: AtomicPtr::new(test_ptr),
        ghost: PhantomData,
    };
    
    let mut output = core::fmt::Formatter::new();
    let result = once_box.fmt(&mut output);
    
    assert!(result.is_ok());
}

#[test]
fn test_once_box_fmt_with_null_pointer() {
    use super::OnceBox;
    use core::ptr;
    
    struct TestData;
    
    let once_box = OnceBox {
        inner: AtomicPtr::new(ptr::null_mut()),
        ghost: PhantomData,
    };

    let mut output = core::fmt::Formatter::new();
    let result = once_box.fmt(&mut output);
    
    assert!(result.is_ok());
}

