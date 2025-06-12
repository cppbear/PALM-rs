// Answer 0

#[test]
fn test_get_or_try_init_success() {
    use once_cell::race::Race;

    struct MyCell {
        inner: Race<usize>,
    }

    impl MyCell {
        fn new() -> Self {
            MyCell {
                inner: Race::new(),
            }
        }

        fn to_usize(value: bool) -> usize {
            if value { 1 } else { 0 }
        }

        fn from_usize(value: usize) -> bool {
            value != 0
        }

        fn get_or_try_init<F, E>(&self, f: F) -> Result<bool, E>
        where
            F: FnOnce() -> Result<bool, E>,
        {
            self.inner.get_or_try_init(|| f().map(Self::to_usize)).map(Self::from_usize)
        }
    }

    let cell = MyCell::new();
    let result = cell.get_or_try_init(|| Ok(true)).unwrap();
    assert_eq!(result, true);
}

#[test]
fn test_get_or_try_init_failure() {
    use once_cell::race::Race;

    struct MyCell {
        inner: Race<usize>,
    }

    impl MyCell {
        fn new() -> Self {
            MyCell {
                inner: Race::new(),
            }
        }

        fn to_usize(value: bool) -> usize {
            if value { 1 } else { 0 }
        }

        fn from_usize(value: usize) -> bool {
            value != 0
        }

        fn get_or_try_init<F, E>(&self, f: F) -> Result<bool, E>
        where
            F: FnOnce() -> Result<bool, E>,
        {
            self.inner.get_or_try_init(|| f().map(Self::to_usize)).map(Self::from_usize)
        }
    }

    let cell = MyCell::new();
    let result: Result<bool, &str> = cell.get_or_try_init(|| Err("initializer failed"));
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), "initializer failed");
}

#[test]
fn test_get_or_try_init_multiple_success() {
    use once_cell::race::Race;
    use std::sync::Arc;
    use std::thread;

    struct MyCell {
        inner: Race<usize>,
    }

    impl MyCell {
        fn new() -> Self {
            MyCell {
                inner: Race::new(),
            }
        }

        fn to_usize(value: bool) -> usize {
            if value { 1 } else { 0 }
        }

        fn from_usize(value: usize) -> bool {
            value != 0
        }

        fn get_or_try_init<F, E>(&self, f: F) -> Result<bool, E>
        where
            F: FnOnce() -> Result<bool, E>,
        {
            self.inner.get_or_try_init(|| f().map(Self::to_usize)).map(Self::from_usize)
        }
    }

    let cell = Arc::new(MyCell::new());

    let mut handles = vec![];
    for _ in 0..10 {
        let cell_clone = Arc::clone(&cell);
        let handle = thread::spawn(move || {
            cell_clone.get_or_try_init(|| Ok(true)).unwrap()
        });
        handles.push(handle);
    }

    for handle in handles {
        let result = handle.join().unwrap();
        assert_eq!(result, true);
    }
}

