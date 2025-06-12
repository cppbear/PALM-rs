// Answer 0

#[derive(Debug)]
struct OnceCell<T>(Imp<T>);

#[derive(Debug)]
struct Imp<T> {
    value: Option<T>,
}

impl<T> Imp<T> {
    pub const fn with_value(value: T) -> Self {
        Imp { value: Some(value) }
    }
}

#[test]
fn test_with_value_initializes_cell() {
    let cell: OnceCell<i32> = with_value(42);
    assert!(cell.0.value.is_some());
    assert_eq!(cell.0.value.unwrap(), 42);
}

#[test]
fn test_with_value_initializes_cell_with_string() {
    let cell: OnceCell<&str> = with_value("Hello");
    assert!(cell.0.value.is_some());
    assert_eq!(cell.0.value.unwrap(), "Hello");
}

#[test]
fn test_with_value_initializes_cell_with_floating_point() {
    let cell: OnceCell<f64> = with_value(3.14);
    assert!(cell.0.value.is_some());
    assert_eq!(cell.0.value.unwrap(), 3.14);
}

#[test]
#[should_panic]
fn test_with_value_creates_none_cell() {
    let _cell: OnceCell<i32> = with_value(0);
    // This test case is here to represent a should panic case
}

