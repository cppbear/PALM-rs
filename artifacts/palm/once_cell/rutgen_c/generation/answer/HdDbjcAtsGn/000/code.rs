// Answer 0

#[test]
fn test_get_or_init_with_initialization() {
    struct TestCell {
        data: OnceCell<i32>,
    }

    impl TestCell {
        fn new() -> Self {
            TestCell {
                data: OnceCell::new(),
            }
        }
    }

    let cell = TestCell::new();
    let value = cell.data.get_or_init(|| 42);
    assert_eq!(value, &42);
}

#[test]
fn test_get_or_init_reinitializes() {
    struct TestCell {
        data: OnceCell<i32>,
    }

    impl TestCell {
        fn new() -> Self {
            TestCell {
                data: OnceCell::new(),
            }
        }
    }

    let cell = TestCell::new();
    let first_value = cell.data.get_or_init(|| 100);
    assert_eq!(first_value, &100);

    let second_value = cell.data.get_or_init(|| panic!("This should not be initialized"));
    assert_eq!(second_value, &100); // Should not panic, so values are the same
}

#[should_panic]
#[test]
fn test_get_or_init_panics_on_reentrant_initialization() {
    struct TestCell {
        data: OnceCell<i32>,
    }

    impl TestCell {
        fn new() -> Self {
            TestCell {
                data: OnceCell::new(),
            }
        }
    }

    let cell = TestCell::new();
    let _value = cell.data.get_or_init(|| {
        cell.data.get_or_init(|| 200); // Reentrant call, should panic
        300
    });
}

