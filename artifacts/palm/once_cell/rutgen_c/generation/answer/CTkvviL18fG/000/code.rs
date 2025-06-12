// Answer 0

#[test]
fn test_get_or_init_with_initialization() {
    struct TestCell {
        inner: OnceCell<i32>,
    }

    impl TestCell {
        fn new() -> Self {
            TestCell {
                inner: OnceCell::new(),
            }
        }
    }

    let cell = TestCell::new();
    let value = cell.inner.get_or_init(|| 42);
    assert_eq!(value, &42);
}

#[test]
fn test_get_or_init_with_retrieval() {
    struct TestCell {
        inner: OnceCell<i32>,
    }

    impl TestCell {
        fn new() -> Self {
            TestCell {
                inner: OnceCell::new(),
            }
        }
    }

    let mut cell = TestCell::new();
    let value = cell.inner.get_or_init(|| 42);
    assert_eq!(value, &42);
    let second_value = cell.inner.get_or_init(|| 99);
    assert_eq!(second_value, &42);
}

#[should_panic]
fn test_get_or_init_panic() {
    struct TestCell {
        inner: OnceCell<i32>,
    }

    impl TestCell {
        fn new() -> Self {
            TestCell {
                inner: OnceCell::new(),
            }
        }
    }

    let mut cell = TestCell::new();
    let _value = cell.inner.get_or_init(|| panic!("initialization panic"));
}

#[test]
fn test_get_or_init_with_return() {
    struct TestCell {
        inner: OnceCell<i32>,
    }

    impl TestCell {
        fn new() -> Self {
            TestCell {
                inner: OnceCell::new(),
            }
        }
    }

    let mut cell = TestCell::new();
    let value = cell.inner.get_or_init(|| 10);
    assert_eq!(value, &10);
    let returned_value = cell.inner.get_or_init(|| 20);
    assert_eq!(returned_value, &10);    
}

