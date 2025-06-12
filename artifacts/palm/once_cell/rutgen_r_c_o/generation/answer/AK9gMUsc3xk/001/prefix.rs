// Answer 0

#[test]
fn test_once_cell_new() {
    let cell: OnceCell<Option<i32>> = OnceCell::new();
}

#[test]
fn test_once_cell_new_with_string() {
    let cell: OnceCell<Option<String>> = OnceCell::new();
}

#[test]
fn test_once_cell_new_with_custom_struct() {
    struct MyStruct;
    let cell: OnceCell<Option<MyStruct>> = OnceCell::new();
}

#[test]
fn test_once_cell_new_with_reference() {
    let val = 42;
    let cell: OnceCell<Option<&i32>> = OnceCell::new();
}

