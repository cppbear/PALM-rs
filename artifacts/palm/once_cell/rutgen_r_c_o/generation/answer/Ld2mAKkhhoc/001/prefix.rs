// Answer 0

#[test]
fn test_set_err_when_full() {
    let cell = OnceCell::new();

    // First insert a value
    let _ = cell.set(92);
    
    // Attempt to set the same value again, expecting an Err
    let result = cell.set(92);
}

#[test]
fn test_set_err_with_different_initial_value() {
    let cell = OnceCell::new();

    // First insert a value
    let _ = cell.set(42);
    
    // Attempt to set a different value, expecting an Err with the new value
    let result = cell.set(42);
}

#[test]
fn test_set_err_when_cell_already_set() {
    let cell = OnceCell::new();
    
    // First insert a value
    let _ = cell.set(100);
    
    // Attempt to set the same value again, expecting an Err
    let result = cell.set(100);
}

