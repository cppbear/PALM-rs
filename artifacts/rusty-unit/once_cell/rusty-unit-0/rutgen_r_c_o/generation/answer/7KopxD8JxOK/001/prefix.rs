// Answer 0

#[test]
fn test_once_cell_fmt_with_value_in_range() {
    let once_cell = OnceCell::with_value(5);
    let mut formatter = fmt::Formatter::new();
    let _ = once_cell.fmt(&mut formatter);
}

#[test]
fn test_once_cell_fmt_with_value_exact_min() {
    let once_cell = OnceCell::with_value(1);
    let mut formatter = fmt::Formatter::new();
    let _ = once_cell.fmt(&mut formatter);
}

#[test]
fn test_once_cell_fmt_with_value_exact_max() {
    let once_cell = OnceCell::with_value(10);
    let mut formatter = fmt::Formatter::new();
    let _ = once_cell.fmt(&mut formatter);
}

#[test]
fn test_once_cell_fmt_with_value_edge_case() {
    let once_cell = OnceCell::with_value(7);
    let mut formatter = fmt::Formatter::new();
    let _ = once_cell.fmt(&mut formatter);
}

