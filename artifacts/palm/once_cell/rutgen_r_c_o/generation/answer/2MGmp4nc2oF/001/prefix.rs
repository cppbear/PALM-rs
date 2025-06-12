// Answer 0

#[test]
fn test_once_cell_string_empty() {
    let cell: OnceCell<String> = OnceCell::new();
    let _result = cell.into_inner();
}

#[test]
fn test_once_cell_string_with_value() {
    let mut cell = OnceCell::new();
    let _ = cell.set("test".to_string()).unwrap();
    let _result = cell.into_inner();
}

#[test]
fn test_once_cell_usize_empty() {
    let cell: OnceCell<usize> = OnceCell::new();
    let _result = cell.into_inner();
}

#[test]
fn test_once_cell_usize_with_value() {
    let mut cell = OnceCell::new();
    let _ = cell.set(42).unwrap();
    let _result = cell.into_inner();
}

#[test]
fn test_once_cell_vec_i32_empty() {
    let cell: OnceCell<Vec<i32>> = OnceCell::new();
    let _result = cell.into_inner();
}

#[test]
fn test_once_cell_vec_i32_with_value() {
    let mut cell = OnceCell::new();
    let _ = cell.set(vec![1, 2, 3]).unwrap();
    let _result = cell.into_inner();
}

