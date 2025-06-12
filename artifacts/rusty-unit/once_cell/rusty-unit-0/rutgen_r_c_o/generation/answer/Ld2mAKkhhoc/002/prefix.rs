// Answer 0

#[test]
fn test_once_cell_set_empty() {
    let cell = OnceCell::new();
    let result = cell.set(1);
}

#[test]
fn test_once_cell_set_non_empty() {
    let cell = OnceCell::new();
    let _ = cell.set(1);
    let result = cell.set(2);
}

#[test]
fn test_once_cell_set_multiple_values() {
    let cell = OnceCell::new();
    let _ = cell.set(1);
    let first_result = cell.set(2);
    let second_result = cell.set(3);
}

#[test]
fn test_once_cell_set_same_value() {
    let cell = OnceCell::new();
    let _ = cell.set(1);
    let result = cell.set(1);
}

