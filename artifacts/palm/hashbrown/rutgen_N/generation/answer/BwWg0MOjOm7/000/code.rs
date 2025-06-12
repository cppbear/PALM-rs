// Answer 0

#[test]
fn test_offset_from_same_pointer() {
    let value = 10;
    let ptr = &value as *const i32;
    let result = unsafe { offset_from(ptr, ptr) };
    assert_eq!(result, 0);
}

#[test]
fn test_offset_from_forward() {
    let arr = [1, 2, 3, 4, 5];
    let ptr_a = &arr[1] as *const i32; // points to 2
    let ptr_b = &arr[0] as *const i32; // points to 1
    let result = unsafe { offset_from(ptr_a, ptr_b) };
    assert_eq!(result, 1);
}

#[test]
fn test_offset_from_backward() {
    let arr = [1, 2, 3, 4, 5];
    let ptr_a = &arr[0] as *const i32; // points to 1
    let ptr_b = &arr[1] as *const i32; // points to 2
    let result = unsafe { offset_from(ptr_a, ptr_b) };
    assert_eq!(result, usize::max_value());
}

