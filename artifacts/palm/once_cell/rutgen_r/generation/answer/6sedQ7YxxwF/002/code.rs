// Answer 0

#[test]
fn test_get_or_try_init_empty_cell_success() {
    use std::num::NonZeroUsize;
    use once_cell::sync::OnceCell;

    // Initialize OnceCell
    let cell: OnceCell<NonZeroUsize> = OnceCell::new();

    // Function that initializes the cell successfully
    let result = cell.get_or_try_init(|| {
        NonZeroUsize::new(42).ok_or("Failed to create NonZeroUsize")
    });

    // Verify that the result is Ok with expected value
    assert_eq!(result, Ok(NonZeroUsize::new(42).unwrap()));
    assert_eq!(cell.get(), Some(&NonZeroUsize::new(42).unwrap()));
}

#[test]
fn test_get_or_try_init_empty_cell_failure() {
    use std::num::NonZeroUsize;
    use once_cell::sync::OnceCell;

    // Initialize OnceCell
    let cell: OnceCell<NonZeroUsize> = OnceCell::new();

    // Function that fails to initialize the cell
    let result = cell.get_or_try_init(|| {
        Err("Failed to create NonZeroUsize") // Simulate failure
    });

    // Verify that the result is an Err value
    assert_eq!(result, Err("Failed to create NonZeroUsize"));
    assert_eq!(cell.get(), None); // Cell should remain empty
}

#[test]
fn test_get_or_try_init_multiple_threads() {
    use std::num::NonZeroUsize;
    use once_cell::sync::OnceCell;
    use std::sync::{Arc, Mutex};
    use std::thread;

    // Initialize OnceCell
    let cell = Arc::new(OnceCell::new());

    // Create a vector to hold the handles of the threads
    let mut handles = vec![];

    for _ in 0..10 {
        let cell_clone = Arc::clone(&cell);
        let handle = thread::spawn(move || {
            cell_clone.get_or_try_init(|| {
                NonZeroUsize::new(42).ok_or("Failed to create NonZeroUsize")
            })
        });
        handles.push(handle);
    }

    // Collect results from all threads
    let mut results: Vec<Result<NonZeroUsize, &str>> = vec![];
    for handle in handles {
        results.push(handle.join().unwrap());
    }

    // Verify that all results are Ok with the same value
    for result in results {
        assert_eq!(result, Ok(NonZeroUsize::new(42).unwrap()));
    }
    assert_eq!(cell.get(), Some(&NonZeroUsize::new(42).unwrap()));
}

