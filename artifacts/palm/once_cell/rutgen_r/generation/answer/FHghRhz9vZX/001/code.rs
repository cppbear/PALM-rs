// Answer 0

#[test]
fn test_with_value_initialized_cell() {
    struct OnceCell<T> {
        inner: std::cell::UnsafeCell<Option<T>>,
    }

    const fn with_value<T>(value: T) -> OnceCell<T> {
        OnceCell { inner: std::cell::UnsafeCell::new(Some(value)) }
    }

    let cell: OnceCell<i32> = with_value(42);
    unsafe {
        assert_eq!(*cell.inner.get(), Some(42));
    }
}

#[test]
fn test_with_value_string_cell() {
    struct OnceCell<T> {
        inner: std::cell::UnsafeCell<Option<T>>,
    }

    const fn with_value<T>(value: T) -> OnceCell<T> {
        OnceCell { inner: std::cell::UnsafeCell::new(Some(value)) }
    }

    let cell: OnceCell<&str> = with_value("Hello, world!");
    unsafe {
        assert_eq!(*cell.inner.get(), Some("Hello, world!"));
    }
}

#[test]
fn test_with_value_floating_point_cell() {
    struct OnceCell<T> {
        inner: std::cell::UnsafeCell<Option<T>>,
    }

    const fn with_value<T>(value: T) -> OnceCell<T> {
        OnceCell { inner: std::cell::UnsafeCell::new(Some(value)) }
    }

    let cell: OnceCell<f64> = with_value(3.14);
    unsafe {
        assert_eq!(*cell.inner.get(), Some(3.14));
    }
}

#[test]
#[should_panic]
fn test_with_value_no_value() {
    struct OnceCell<T> {
        inner: std::cell::UnsafeCell<Option<T>>,
    }

    const fn with_value<T>(value: T) -> OnceCell<T> {
        OnceCell { inner: std::cell::UnsafeCell::new(Some(value)) }
    }

    let _: OnceCell<()> = with_value(());
}

