// Answer 0

#[test]
fn test_once_cell_debug_uninit() {
    let once_cell: OnceCell<i32> = OnceCell::new();
    let mut formatter = fmt::Formatter::new();
    once_cell.fmt(&mut formatter);
}

#[test]
fn test_once_cell_debug_uninit_with_value() {
    let once_cell: OnceCell<i32> = OnceCell::with_value(10);
    let mut formatter = fmt::Formatter::new();
    once_cell.get_mut(); // Ensure it's set to initialized

    // Take value to make it uninitialized
    once_cell.take();
    once_cell.fmt(&mut formatter);
}

#[test]
fn test_once_cell_debug_with_different_types() {
    let once_cell: OnceCell<String> = OnceCell::new();
    let mut formatter = fmt::Formatter::new();
    once_cell.fmt(&mut formatter);
}

