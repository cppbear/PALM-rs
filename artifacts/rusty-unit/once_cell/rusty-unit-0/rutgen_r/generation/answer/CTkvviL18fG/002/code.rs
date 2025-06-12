// Answer 0

#[test]
fn test_get_or_init_basic() {
    use once_cell::sync::OnceCell;

    let cell = OnceCell::new();
    let value = cell.get_or_init(|| 42);
    assert_eq!(value, &42);
}

#[test]
fn test_get_or_init_concurrent() {
    use std::sync::{Arc, Mutex};
    use std::thread;
    use once_cell::sync::OnceCell;

    let cell = Arc::new(OnceCell::new());
    let cell_clone = Arc::clone(&cell);
    let handle = thread::spawn(move || {
        cell_clone.get_or_init(|| 100)
    });

    let value = cell.get_or_init(|| 50);
    assert_eq!(value, &100);
    let result = handle.join().unwrap();
    assert_eq!(result, &100);
}

#[should_panic]
#[test]
fn test_get_or_init_panics() {
    use once_cell::sync::OnceCell;

    let cell = OnceCell::new();
    let _value = cell.get_or_init(|| panic!("panic test"));
}

#[test]
fn test_get_or_init_returns_existing_value() {
    use once_cell::sync::OnceCell;

    let cell = OnceCell::new();
    let _initial_value = cell.get_or_init(|| 99);
    let second_value = cell.get_or_init(|| 100);
    assert_eq!(second_value, &99);
}

#[test]
fn test_get_or_init_reentrant_call() {
    use once_cell::sync::OnceCell;

    let cell = OnceCell::new();
    let _first_call = cell.get_or_init(|| 10);
    
    // This call should not panic or deadlock, but may lead to
    // unspecified behavior based on the original implementation.
    let reentrant_value = cell.get_or_init(|| 20);
    assert_eq!(reentrant_value, &10);
}

