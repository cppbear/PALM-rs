// Answer 0

#[test]
fn test_once_cell_with_value() {
    // Testing with a simple integer value
    let cell_int = OnceCell::with_value(42);
    assert_eq!(cell_int.get(), Some(&42));

    // Testing with a string value
    let cell_string = OnceCell::with_value(String::from("Hello, World!"));
    assert_eq!(cell_string.get(), Some(&"Hello, World!".to_string()));

    // Testing with a tuple
    let cell_tuple = OnceCell::with_value((3, 4));
    assert_eq!(cell_tuple.get(), Some(&(3, 4)));

    // Testing with a custom struct
    struct Point {
        x: i32,
        y: i32,
    }
    let cell_point = OnceCell::with_value(Point { x: 10, y: 20 });
    assert_eq!(cell_point.get(), Some(&Point { x: 10, y: 20 }));
}

