// Answer 0

#[test]
fn test_once_cell_creation() {
    struct OnceCell<T> {
        inner: std::cell::UnsafeCell<Option<T>>,
    }

    const fn new<T>() -> OnceCell<T> {
        OnceCell { inner: std::cell::UnsafeCell::new(None) }
    }

    let cell: OnceCell<i32> = new();
    // Check if the cell is initialized as None
    unsafe {
        assert_eq!(*cell.inner.get(), None);
    }
}

#[test]
fn test_once_cell_non_empty() {
    struct OnceCell<T> {
        inner: std::cell::UnsafeCell<Option<T>>,
    }

    const fn new<T>() -> OnceCell<T> {
        OnceCell { inner: std::cell::UnsafeCell::new(None) }
    }

    const fn set<T>(cell: &OnceCell<T>, value: T) {
        unsafe {
            *cell.inner.get() = Some(value);
        }
    }

    let cell: OnceCell<i32> = new();
    set(&cell, 10);
    
    unsafe {
        assert_eq!(*cell.inner.get(), Some(10));
    }
}

