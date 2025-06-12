// Answer 0

#[test]
fn test_new_once_cell() {
    struct OnceCell<T> {
        inner: std::cell::UnsafeCell<Option<T>>,
    }

    const fn new<T>() -> OnceCell<T> {
        OnceCell { inner: std::cell::UnsafeCell::new(None) }
    }

    let cell: OnceCell<i32> = new();
    let inner = unsafe { &*cell.inner.get() };
    assert!(inner.is_none());
}

#[test]
fn test_new_once_cell_with_different_types() {
    struct OnceCell<T> {
        inner: std::cell::UnsafeCell<Option<T>>,
    }

    const fn new<T>() -> OnceCell<T> {
        OnceCell { inner: std::cell::UnsafeCell::new(None) }
    }

    let string_cell: OnceCell<String> = new();
    let inner = unsafe { &*string_cell.inner.get() };
    assert!(inner.is_none());

    let float_cell: OnceCell<f64> = new();
    let inner_float = unsafe { &*float_cell.inner.get() };
    assert!(inner_float.is_none());
}

