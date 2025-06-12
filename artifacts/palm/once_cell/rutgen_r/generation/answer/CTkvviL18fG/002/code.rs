// Answer 0

#[test]
fn test_get_or_init_success() {
    use once_cell::sync::OnceCell;

    let cell = OnceCell::new();
    let value = cell.get_or_init(|| 42);
    assert_eq!(value, &42);
}

#[test]
fn test_get_or_init_already_initialized() {
    use once_cell::sync::OnceCell;

    let cell = OnceCell::new();
    let value1 = cell.get_or_init(|| 55);
    assert_eq!(value1, &55);
    
    // Attempt to re-initialize, should return the same value
    let value2 = cell.get_or_init(|| 99);
    assert_eq!(value2, &55);
}

#[test]
#[should_panic]
fn test_get_or_init_panic() {
    use once_cell::sync::OnceCell;

    let cell = OnceCell::new();
    let _value = cell.get_or_init(|| panic!("This should panic"));
}

#[test]
fn test_get_or_init_thread_safety() {
    use std::sync::Arc;
    use std::thread;
    use once_cell::sync::OnceCell;

    let cell = Arc::new(OnceCell::new());
    let cell_clone = Arc::clone(&cell);
    
    let handles: Vec<_> = (0..10).map(|_| {
        let cell_clone = Arc::clone(&cell_clone);
        thread::spawn(move || {
            cell_clone.get_or_init(|| 100)
        })
    }).collect();

    for handle in handles {
        let value = handle.join().unwrap();
        assert_eq!(value, &100);
    }
}

