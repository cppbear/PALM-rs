// Answer 0

#[test]
fn test_wait_initializes_value() {
    use once_cell::sync::OnceCell;
    use std::sync::Arc;

    let cell = Arc::new(OnceCell::new());
    let t = std::thread::spawn({
        let cell = Arc::clone(&cell);
        move || cell.set(42).unwrap()
    });

    // Initially, the cell should not be initialized
    assert!(cell.get().is_none());

    // Wait for the other thread to set the value
    let value: &u32 = cell.wait();
    
    // Check if the returned value is as expected
    assert_eq!(*value, 42);
    
    t.join().unwrap();
}

#[test]
#[should_panic]
fn test_wait_panics_if_not_set() {
    use once_cell::sync::OnceCell;
    use std::sync::Arc;

    let cell = Arc::new(OnceCell::new());

    // This test should panic because `wait` is called on an unset cell
    let _value: &u32 = cell.wait();
}

