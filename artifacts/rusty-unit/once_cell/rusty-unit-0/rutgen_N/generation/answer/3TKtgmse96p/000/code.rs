// Answer 0

#[test]
fn test_get_or_try_init_success() {
    struct MyCell {
        value: once_cell::sync::OnceCell<i32>,
    }

    impl MyCell {
        fn new() -> Self {
            MyCell {
                value: once_cell::sync::OnceCell::new(),
            }
        }

        fn get_or_try_init(&self) -> Result<&i32, ()> {
            self.value.get_or_try_init(|| Ok(42))
        }
    }

    let cell = MyCell::new();
    let result = cell.get_or_try_init();
    assert_eq!(result, Ok(&42));
    assert_eq!(cell.value.get(), Some(&42));
}

#[test]
fn test_get_or_try_init_failure() {
    struct MyCell {
        value: once_cell::sync::OnceCell<i32>,
    }

    impl MyCell {
        fn new() -> Self {
            MyCell {
                value: once_cell::sync::OnceCell::new(),
            }
        }

        fn get_or_try_init(&self) -> Result<&i32, ()> {
            self.value.get_or_try_init(|| Err(()))
        }
    }

    let cell = MyCell::new();
    let result = cell.get_or_try_init();
    assert_eq!(result, Err(()));
    assert!(cell.value.get().is_none());
}

#[test]
#[should_panic]
fn test_get_or_try_init_panic() {
    struct MyCell {
        value: once_cell::sync::OnceCell<i32>,
    }

    impl MyCell {
        fn new() -> Self {
            MyCell {
                value: once_cell::sync::OnceCell::new(),
            }
        }

        fn get_or_try_init(&self) -> Result<&i32, ()> {
            self.value.get_or_try_init(|| {
                panic!("Initialization failed")
            })
        }
    }

    let cell = MyCell::new();
    let _ = cell.get_or_try_init();
}

