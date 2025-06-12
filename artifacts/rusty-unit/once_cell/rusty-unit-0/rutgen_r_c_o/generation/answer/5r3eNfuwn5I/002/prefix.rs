// Answer 0

#[test]
fn test_set_unique_values() {
    let cell: OnceCell<i32> = OnceCell::new();
    let _ = cell.set(1);
    let _ = cell.set(2);
    let _ = cell.set(3);
}

#[test]
fn test_set_zero_value() {
    let cell: OnceCell<i32> = OnceCell::new();
    let _ = cell.set(0);
}

#[test]
fn test_set_negative_value() {
    let cell: OnceCell<i32> = OnceCell::new();
    let _ = cell.set(-10);
}

#[test]
fn test_set_large_value() {
    let cell: OnceCell<i64> = OnceCell::new();
    let _ = cell.set(1000000);
}

#[test]
fn test_set_char_value() {
    let cell: OnceCell<char> = OnceCell::new();
    let _ = cell.set('a');
}

#[test]
fn test_set_string_value() {
    let cell: OnceCell<String> = OnceCell::new();
    let _ = cell.set(String::from("hello"));
}

