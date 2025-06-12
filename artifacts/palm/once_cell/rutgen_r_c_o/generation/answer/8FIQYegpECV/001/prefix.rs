// Answer 0

#[test]
fn test_set_when_full() {
    struct TestStruct {
        value: i32,
    }

    let once_box = OnceBox::<TestStruct>::with_value(Box::new(TestStruct { value: 10 }));

    let new_value = Box::new(TestStruct { value: 20 });
    let result = once_box.set(new_value);

    // Expected: result should be an Err containing the already set value
}

#[test]
fn test_set_with_diff_value_when_full() {
    struct TestStruct {
        value: i32,
    }

    let once_box = OnceBox::<TestStruct>::with_value(Box::new(TestStruct { value: 30 }));

    let new_value = Box::new(TestStruct { value: 40 });
    let result = once_box.set(new_value);

    // Expected: result should be an Err containing the already set value
}

#[test]
fn test_set_on_non_empty_once_box() {
    struct TestStruct {
        data: String,
    }

    let once_box = OnceBox::<TestStruct>::with_value(Box::new(TestStruct { data: String::from("Initial") }));

    let new_value = Box::new(TestStruct { data: String::from("Another") });
    let result = once_box.set(new_value);

    // Expected: result should be an Err containing the already set value
}

