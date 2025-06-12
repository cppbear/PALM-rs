// Answer 0

#[test]
fn test_once_cell_new() {
    let cell: OnceCell<i32> = OnceCell::new();
}

#[test]
fn test_once_cell_new_empty() {
    let cell: OnceCell<String> = OnceCell::new();
}

#[test]
fn test_once_cell_new_struct() {
    #[derive(Debug, Copy, Clone)]
    struct MyStruct {
        value: i32,
    }
    let cell: OnceCell<MyStruct> = OnceCell::new();
}

#[test]
fn test_once_cell_new_float() {
    let cell: OnceCell<f64> = OnceCell::new();
}

#[test]
fn test_once_cell_new_bool() {
    let cell: OnceCell<bool> = OnceCell::new();
}

