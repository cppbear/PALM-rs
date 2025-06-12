// Answer 0

#[test]
fn test_once_cell_with_value() {
    // Test with a simple value
    let cell = OnceCell::with_value(42);
    let inner_ptr = unsafe { &*cell.inner.get() };
    assert_eq!(*inner_ptr, Some(42));

    // Test with a string
    let string_value = String::from("Hello, world!");
    let cell_string = OnceCell::with_value(string_value.clone());
    let inner_string_ptr = unsafe { &*cell_string.inner.get() };
    assert_eq!(*inner_string_ptr, Some(string_value));

    // Test with a float
    let float_value = 9.81;
    let cell_float = OnceCell::with_value(float_value);
    let inner_float_ptr = unsafe { &*cell_float.inner.get() };
    assert_eq!(*inner_float_ptr, Some(float_value));

    // Test with a struct
    #[derive(Debug, PartialEq)]
    struct TestStruct {
        value: i32,
    }
    let struct_value = TestStruct { value: 10 };
    let cell_struct = OnceCell::with_value(struct_value.clone());
    let inner_struct_ptr = unsafe { &*cell_struct.inner.get() };
    assert_eq!(*inner_struct_ptr, Some(struct_value));
}

#[test]
fn test_once_cell_with_value_boundary() {
    // Test with the maximum value of i32
    let max_cell = OnceCell::with_value(i32::MAX);
    let inner_max_ptr = unsafe { &*max_cell.inner.get() };
    assert_eq!(*inner_max_ptr, Some(i32::MAX));

    // Test with the minimum value of i32
    let min_cell = OnceCell::with_value(i32::MIN);
    let inner_min_ptr = unsafe { &*min_cell.inner.get() };
    assert_eq!(*inner_min_ptr, Some(i32::MIN));
}

