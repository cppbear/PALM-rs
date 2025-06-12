// Answer 0

#[test]
fn test_get_none_when_empty() {
    struct Cell<T> {
        inner: std::cell::UnsafeCell<Option<T>>,
    }

    impl<T> Cell<T> {
        pub fn new() -> Self {
            Cell {
                inner: std::cell::UnsafeCell::new(None),
            }
        }

        pub fn get(&self) -> Option<&T> {
            unsafe { &*self.inner.get() }.as_ref()
        }
    }

    let cell: Cell<i32> = Cell::new();
    assert_eq!(cell.get(), None);
}

#[test]
fn test_get_some_when_initialized() {
    struct Cell<T> {
        inner: std::cell::UnsafeCell<Option<T>>,
    }

    impl<T> Cell<T> {
        pub fn new() -> Self {
            Cell {
                inner: std::cell::UnsafeCell::new(None),
            }
        }

        pub fn set(&self, value: T) {
            unsafe {
                *self.inner.get() = Some(value);
            }
        }

        pub fn get(&self) -> Option<&T> {
            unsafe { &*self.inner.get() }.as_ref()
        }
    }

    let cell: Cell<i32> = Cell::new();
    cell.set(42);
    assert_eq!(cell.get(), Some(&42));
}

