// Answer 0

#[test]
fn test_set_with_empty_cell() {
    let once_box: OnceBox<i32> = OnceBox::new();
    let value = Box::new(42);
    let result = once_box.set(value);
    assert_eq!(result, Ok(()));
}

#[test]
fn test_set_with_full_cell() {
    let mut once_box: OnceBox<i32> = OnceBox::new();
    let value1 = Box::new(42);
    let _ = once_box.set(value1); // Set the first value
    let value2 = Box::new(100);
    let result = once_box.set(value2); // Attempt to set a second value
    assert!(result.is_err());
}

#[test]
fn test_set_with_full_cell_return_value() {
    let mut once_box: OnceBox<i32> = OnceBox::new();
    let value1 = Box::new(42);
    let _ = once_box.set(value1); // Set the first value
    let value2 = Box::new(100);
    let result = once_box.set(value2); // Attempt to set a second value
    if let Err(err_value) = result {
        assert_eq!(*err_value, 100); // Check if the returned value is the one that was attempted to be set
    } else {
        panic!("Expected an error when setting a second value.");
    }
}

