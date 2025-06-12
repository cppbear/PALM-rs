// Answer 0

#[test]
fn test_get_or_init_with_valid_function() {
    struct MyCell {
        inner: OnceCell<i32>,
    }

    let cell = MyCell { inner: OnceCell::new() };
    let value = cell.inner.get_or_init(|| 42);
    assert_eq!(value, &42);
}

#[test]
fn test_get_or_init_with_value_already_set() {
    struct MyCell {
        inner: OnceCell<i32>,
    }

    let mut cell = MyCell { inner: OnceCell::new() };
    let _ = cell.inner.get_or_init(|| 42);
    let value = cell.inner.get_or_init(|| panic!("This should not run."));
    assert_eq!(value, &42);
}

#[test]
#[should_panic]
fn test_get_or_init_panics() {
    struct MyCell {
        inner: OnceCell<i32>,
    }

    let mut cell = MyCell { inner: OnceCell::new() };
    let _ = cell.inner.get_or_init(|| panic!("Triggered panic"));
}

#[test]
fn test_get_or_init_with_function_returning_zero() {
    struct MyCell {
        inner: OnceCell<i32>,
    }

    let cell = MyCell { inner: OnceCell::new() };
    let value = cell.inner.get_or_init(|| 0);
    assert_eq!(value, &0);
}

#[test]
fn test_get_or_init_with_empty_initialization_function() {
    struct MyCell {
        inner: OnceCell<String>,
    }

    let cell = MyCell { inner: OnceCell::new() };
    let value = cell.inner.get_or_init(|| String::from("Initialized"));
    assert_eq!(value, &"Initialized".to_string());
}

