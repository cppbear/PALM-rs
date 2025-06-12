// Answer 0

#[test]
fn test_try_insert_initial_state() {
    struct OnceCell<T> {
        inner: std::cell::UnsafeCell<Option<T>>,
    }

    impl<T> OnceCell<T> {
        fn new() -> Self {
            OnceCell {
                inner: std::cell::UnsafeCell::new(None),
            }
        }

        fn get(&self) -> Option<&T> {
            unsafe { &*self.inner.get() }.as_ref()
        }

        pub fn try_insert(&self, value: T) -> Result<&T, (&T, T)> {
            if let Some(old) = self.get() {
                return Err((old, value));
            }

            let slot = unsafe { &mut *self.inner.get() };
            *slot = Some(value);
            Ok(unsafe { slot.as_ref().unwrap_unchecked() })
        }
    }

    let cell = OnceCell::new();
    assert!(cell.get().is_none());

    assert_eq!(cell.try_insert(92), Ok(&92));
    assert_eq!(cell.try_insert(62), Err((&92, 62)));

    assert!(cell.get().is_some());
}

#[test]
fn test_try_insert_twice() {
    struct OnceCell<T> {
        inner: std::cell::UnsafeCell<Option<T>>,
    }

    impl<T> OnceCell<T> {
        fn new() -> Self {
            OnceCell {
                inner: std::cell::UnsafeCell::new(None),
            }
        }

        fn get(&self) -> Option<&T> {
            unsafe { &*self.inner.get() }.as_ref()
        }

        pub fn try_insert(&self, value: T) -> Result<&T, (&T, T)> {
            if let Some(old) = self.get() {
                return Err((old, value));
            }

            let slot = unsafe { &mut *self.inner.get() };
            *slot = Some(value);
            Ok(unsafe { slot.as_ref().unwrap_unchecked() })
        }
    }

    let cell = OnceCell::new();
    assert!(cell.get().is_none());

    assert_eq!(cell.try_insert(100), Ok(&100));
    assert_eq!(cell.try_insert(200), Err((&100, 200)));
} 

#[test]
fn test_try_insert_reference() {
    struct OnceCell<T> {
        inner: std::cell::UnsafeCell<Option<T>>,
    }

    impl<T> OnceCell<T> {
        fn new() -> Self {
            OnceCell {
                inner: std::cell::UnsafeCell::new(None),
            }
        }

        fn get(&self) -> Option<&T> {
            unsafe { &*self.inner.get() }.as_ref()
        }

        pub fn try_insert(&self, value: T) -> Result<&T, (&T, T)> {
            if let Some(old) = self.get() {
                return Err((old, value));
            }

            let slot = unsafe { &mut *self.inner.get() };
            *slot = Some(value);
            Ok(unsafe { slot.as_ref().unwrap_unchecked() })
        }
    }

    let cell = OnceCell::new();
    let value = String::from("Hello");
    
    assert_eq!(cell.try_insert(value), Ok(&String::from("Hello")));
}

