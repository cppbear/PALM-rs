// Answer 0

#[test]
fn test_drop_function_safely() {
    struct TestData {
        value: i32,
    }

    let mut data: Box<TestData> = Box::new(TestData { value: 42 });
    let ptr = Box::into_raw(data);
    
    unsafe {
        // Calling the drop function directly on the pointed data
        ptr::drop_in_place(ptr);
    }
}

#[test]
#[should_panic]
fn test_drop_function_double_drop() {
    struct TestData {
        value: i32,
    }

    let data: Box<TestData> = Box::new(TestData { value: 42 });
    let ptr = Box::into_raw(data);

    // Calling drop_in_place twice causes a panic due to double drop
    unsafe {
        ptr::drop_in_place(ptr);
    }
    unsafe {
        // This will panic as the data has already been dropped
        ptr::drop_in_place(ptr);
    }
}

