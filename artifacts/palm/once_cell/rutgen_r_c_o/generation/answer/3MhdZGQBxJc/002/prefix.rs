// Answer 0

#[test]
fn test_once_cell_fmt_uninit() {
    let cell: OnceCell<i32> = OnceCell::new();
    let mut buffer = Vec::new();
    {
        let formatter = &mut fmt::Formatter::new(&mut buffer);
        cell.fmt(formatter);
    }
}

#[test]
fn test_once_cell_fmt_uninit_debug_trait() {
    let cell: OnceCell<String> = OnceCell::new();
    let mut buffer = Vec::new();
    {
        let formatter = &mut fmt::Formatter::new(&mut buffer);
        cell.fmt(formatter);
    }
}

#[test]
fn test_once_cell_fmt_uninit_float() {
    let cell: OnceCell<f64> = OnceCell::new();
    let mut buffer = Vec::new();
    {
        let formatter = &mut fmt::Formatter::new(&mut buffer);
        cell.fmt(formatter);
    }
}

#[test]
fn test_once_cell_fmt_uninit_char() {
    let cell: OnceCell<char> = OnceCell::new();
    let mut buffer = Vec::new();
    {
        let formatter = &mut fmt::Formatter::new(&mut buffer);
        cell.fmt(formatter);
    }
}

