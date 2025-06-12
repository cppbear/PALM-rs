// Answer 0

#[test]
fn test_get_or_init_with_valid_non_zero_value() {
    use std::num::NonZeroUsize;
    use std::sync::OnceCell;

    // Create an instance of OnceCell to test
    let cell: OnceCell<NonZeroUsize> = OnceCell::new();

    // Function to initialize the cell with a valid NonZeroUsize
    let initialize = || NonZeroUsize::new(1).unwrap();

    // Call get_or_init and acquire the value
    let value = cell.get_or_init(initialize);
    
    // Check that the value returned is indeed NonZeroUsize with value 1
    assert_eq!(value.get(), 1);
}

#[test]
#[should_panic]
fn test_get_or_init_with_zero_value() {
    use std::num::NonZeroUsize;
    use std::sync::OnceCell;

    // Create an instance of OnceCell to test
    let cell: OnceCell<NonZeroUsize> = OnceCell::new();

    // Function to initialize the cell with a zero value (should panic)
    let initialize = || NonZeroUsize::new(0).unwrap();

    // Call get_or_init which should panic since zero is not a valid NonZeroUsize
    let _ = cell.get_or_init(initialize);
}

#[test]
fn test_get_or_init_multiple_threads() {
    use std::num::NonZeroUsize;
    use std::sync::{Arc, OnceCell};
    use std::thread;

    let cell: Arc<OnceCell<NonZeroUsize>> = Arc::new(OnceCell::new());

    let initialize = || NonZeroUsize::new(2).unwrap();
    let mut handles = vec![];

    for _ in 0..10 {
        let cell_clone = Arc::clone(&cell);
        let handle = thread::spawn(move || {
            cell_clone.get_or_init(initialize);
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    // Ensure the cell is initialized and equals NonZeroUsize(2)
    let value = cell.get().unwrap();
    assert_eq!(value.get(), 2);
}

