// Answer 0

#[test]
fn test_set_when_cell_is_full() {
    struct TestStruct {
        value: i32,
    }

    let once_box = OnceBox::new();
    let initial_value = Box::new(TestStruct { value: 42 });

    // First, set an initial value in the OnceBox to make it "full"
    let _ = once_box.set(initial_value.clone()).unwrap();

    // Now attempt to set a new value, which should return an Err
    let new_value = Box::new(TestStruct { value: 84 });
    let result = once_box.set(new_value);

    match result {
        Err(err_value) => {
            // Ensure that the original value is returned in the Err variant
            let err_value = unsafe { Box::from_raw(err_value.as_mut_ptr()) };
            assert_eq!(err_value.value, 42);
        }
        _ => panic!("Expected Err(value) when cell is full."),
    }
}

