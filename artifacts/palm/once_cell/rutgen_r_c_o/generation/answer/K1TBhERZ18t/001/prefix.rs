// Answer 0

#[test]
fn test_with_value_non_null() {
    let value = Box::new(42);
    let once_box = OnceBox::with_value(value);
}

#[test]
fn test_with_value_string() {
    let value = Box::new(String::from("test"));
    let once_box = OnceBox::with_value(value);
}

#[test]
fn test_with_value_empty_string() {
    let value = Box::new(String::from(""));
    let once_box = OnceBox::with_value(value);
}

#[test]
fn test_with_value_large_boxed_data() {
    let value = Box::new(vec![0; 1_000_000]); // Large vector
    let once_box = OnceBox::with_value(value);
}

#[test]
fn test_with_value_nan() {
    let value = Box::new(f32::NAN);
    let once_box = OnceBox::with_value(value);
}

#[test]
fn test_with_value_negative_value() {
    let value = Box::new(-100);
    let once_box = OnceBox::with_value(value);
}

#[test]
fn test_with_value_struct() {
    #[derive(Debug)]
    struct MyStruct {
        x: i32,
        y: i32,
    }
    let value = Box::new(MyStruct { x: 1, y: 2 });
    let once_box = OnceBox::with_value(value);
}

