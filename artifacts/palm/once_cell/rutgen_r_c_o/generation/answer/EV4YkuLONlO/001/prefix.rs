// Answer 0

#[test]
fn test_once_box_fmt_null_pointer() {
    let once_box: OnceBox<u32> = OnceBox {
        inner: AtomicPtr::new(ptr::null_mut()),
        ghost: PhantomData,
    };
    let mut output = core::fmt::Formatter::new();
    let _ = once_box.fmt(&mut output);
}

#[test]
fn test_once_box_fmt_valid_pointer() {
    let value: Box<u32> = Box::new(42);
    let once_box: OnceBox<u32> = OnceBox {
        inner: AtomicPtr::new(Box::into_raw(value)),
        ghost: PhantomData,
    };
    let mut output = core::fmt::Formatter::new();
    let _ = once_box.fmt(&mut output);
}

#[test]
fn test_once_box_fmt_multiple_times() {
    let value1: Box<u32> = Box::new(1);
    let value2: Box<u32> = Box::new(2);
    let once_box1: OnceBox<u32> = OnceBox {
        inner: AtomicPtr::new(Box::into_raw(value1)),
        ghost: PhantomData,
    };
    let once_box2: OnceBox<u32> = OnceBox {
        inner: AtomicPtr::new(Box::into_raw(value2)),
        ghost: PhantomData,
    };
    let mut output1 = core::fmt::Formatter::new();
    let mut output2 = core::fmt::Formatter::new();
    let _ = once_box1.fmt(&mut output1);
    let _ = once_box2.fmt(&mut output2);
}

#[should_panic]
#[test]
fn test_once_box_fmt_uninitialized() {
    let once_box: OnceBox<u32> = OnceBox {
        inner: AtomicPtr::new(0 as *mut u32),
        ghost: PhantomData,
    };
    let mut output = core::fmt::Formatter::new();
    let _ = once_box.fmt(&mut output);
}

