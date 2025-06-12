// Answer 0

#[test]
fn test_fmt_with_some_value() {
    use std::sync::atomic::{AtomicPtr, Ordering};
    use std::fmt;

    struct OnceBox {
        inner: AtomicPtr<i32>,
    }

    impl fmt::Debug for OnceBox {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "OnceBox({:?})", self.inner.load(Ordering::Relaxed))
        }
    }

    let value = Box::new(42);
    let ptr = Box::into_raw(value);
    let once_box = OnceBox { inner: AtomicPtr::new(ptr) };

    let mut output = String::new();
    let result = write!(&mut output, "{:?}", once_box);
    
    assert!(result.is_ok());
    assert_eq!(output, "OnceBox(0x...),", output);
}

#[test]
fn test_fmt_with_none_value() {
    use std::sync::atomic::{AtomicPtr, Ordering};
    use std::fmt;

    struct OnceBox {
        inner: AtomicPtr<i32>,
    }

    impl fmt::Debug for OnceBox {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "OnceBox({:?})", self.inner.load(Ordering::Relaxed))
        }
    }

    let once_box = OnceBox { inner: AtomicPtr::null() };

    let mut output = String::new();
    let result = write!(&mut output, "{:?}", once_box);
    
    assert!(result.is_ok());
    assert_eq!(output, "OnceBox(0x0)", output);
}

