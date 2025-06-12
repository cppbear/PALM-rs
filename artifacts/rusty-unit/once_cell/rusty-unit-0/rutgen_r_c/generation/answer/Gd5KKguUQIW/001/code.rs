// Answer 0

#[test]
fn test_deref_mut_initializes_and_returns_value() {
    use core::cell::Cell;

    struct OnceCell<T> {
        inner: core::cell::UnsafeCell<Option<T>>,
    }

    impl<T> OnceCell<T> {
        fn with_value(value: T) -> Self {
            OnceCell {
                inner: core::cell::UnsafeCell::new(Some(value)),
            }
        }
        
        fn get_mut(&self) -> &mut Option<T> {
            unsafe { &mut *self.inner.get() }
        }
    }
    
    struct Lazy<T, F = fn() -> T> {
        cell: OnceCell<T>,
        init: Cell<Option<F>>,
    }

    impl<T, F: FnOnce() -> T> Lazy<T, F> {
        pub fn force_mut(this: &mut Lazy<T, F>) -> &mut T {
            if this.cell.get_mut().is_none() {
                let value = match this.init.get_mut().take() {
                    Some(f) => f(),
                    None => panic!("Lazy instance has previously been poisoned"),
                };
                this.cell = OnceCell::with_value(value);
            }
            this.cell.get_mut().as_mut().unwrap()
        }
    }

    let init_fn = || 42;
    let mut lazy_instance = Lazy {
        cell: OnceCell::with_value(None),
        init: Cell::new(Some(init_fn)),
    };

    let value: &mut i32 = lazy_instance.force_mut();
    assert_eq!(*value, 42);
}

#[test]
#[should_panic(expected = "Lazy instance has previously been poisoned")]
fn test_deref_mut_panics_when_poisoned() {
    use core::cell::Cell;

    struct OnceCell<T> {
        inner: core::cell::UnsafeCell<Option<T>>,
    }

    impl<T> OnceCell<T> {
        fn with_value(value: T) -> Self {
            OnceCell {
                inner: core::cell::UnsafeCell::new(Some(value)),
            }
        }
        
        fn get_mut(&self) -> &mut Option<T> {
            unsafe { &mut *self.inner.get() }
        }
    }
    
    struct Lazy<T, F = fn() -> T> {
        cell: OnceCell<T>,
        init: Cell<Option<F>>,
    }

    impl<T, F: FnOnce() -> T> Lazy<T, F> {
        pub fn force_mut(this: &mut Lazy<T, F>) -> &mut T {
            if this.cell.get_mut().is_none() {
                let value = match this.init.get_mut().take() {
                    Some(f) => f(),
                    None => panic!("Lazy instance has previously been poisoned"),
                };
                this.cell = OnceCell::with_value(value);
            }
            this.cell.get_mut().as_mut().unwrap()
        }
    }

    let mut lazy_instance = Lazy {
        cell: OnceCell::with_value(None),
        init: Cell::new(None), // Simulating a poisoned state
    };

    let _value: &mut i32 = lazy_instance.force_mut();
}

