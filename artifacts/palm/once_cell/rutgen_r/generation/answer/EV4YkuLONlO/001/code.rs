// Answer 0

#[test]
fn test_fmt_valid() {
    use std::sync::{Arc, Mutex};
    use std::sync::atomic::{AtomicPtr, Ordering};
    use std::fmt::{self, Debug};

    struct OnceBox<T: Debug> {
        inner: AtomicPtr<T>,
    }

    impl<T: Debug> OnceBox<T> {
        fn new(value: T) -> OnceBox<T> {
            let boxed = Box::new(value);
            OnceBox {
                inner: AtomicPtr::new(Box::into_raw(boxed)),
            }
        }

        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "OnceBox({:?})", unsafe { &*self.inner.load(Ordering::Relaxed) })
        }
    }

    let value = Arc::new(Mutex::new(10));
    let once_box = OnceBox::new(value.clone());

    let mut output = String::new();
    let result = once_box.fmt(&mut fmt::Formatter::new(&mut output));
    assert!(result.is_ok());
    assert!(output.contains("OnceBox(Arc(Mutex { .. }))"));
}

#[test]
#[should_panic]
fn test_fmt_panics_on_null() {
    use std::sync::AtomicPtr;
    use std::fmt::{self, Debug};

    struct OnceBox<T: Debug> {
        inner: AtomicPtr<T>,
    }

    impl<T: Debug> OnceBox<T> {
        fn new() -> OnceBox<T> {
            OnceBox {
                inner: AtomicPtr::new(std::ptr::null_mut()),
            }
        }

        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "OnceBox({:?})", unsafe { &*self.inner.load(Ordering::Relaxed) })
        }
    }

    let once_box: OnceBox<i32> = OnceBox::new();
    let _ = once_box.fmt(&mut fmt::Formatter::new());
}

#[test]
fn test_fmt_different_types() {
    use std::sync::atomic::{AtomicPtr, Ordering};
    use std::fmt::{self, Debug};

    struct OnceBox<T: Debug> {
        inner: AtomicPtr<T>,
    }

    impl<T: Debug> OnceBox<T> {
        fn new(value: T) -> OnceBox<T> {
            let boxed = Box::new(value);
            OnceBox {
                inner: AtomicPtr::new(Box::into_raw(boxed)),
            }
        }

        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "OnceBox({:?})", unsafe { &*self.inner.load(Ordering::Relaxed) })
        }
    }

    let once_box_int = OnceBox::new(42);
    let mut output = String::new();
    let result_int = once_box_int.fmt(&mut fmt::Formatter::new(&mut output));
    assert!(result_int.is_ok());
    assert!(output.contains("OnceBox(42)"));

    let once_box_string = OnceBox::new("Hello, World!".to_string());
    let mut output_string = String::new();
    let result_string = once_box_string.fmt(&mut fmt::Formatter::new(&mut output_string));
    assert!(result_string.is_ok());
    assert!(output_string.contains("OnceBox(\"Hello, World!\""));
}

