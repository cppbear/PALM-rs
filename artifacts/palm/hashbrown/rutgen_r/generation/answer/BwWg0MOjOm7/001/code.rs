// Answer 0

#[test]
fn test_offset_from_different_memory_locations() {
    let a = 10;
    let b = 20;

    let ptr_a: *const i32 = &a;
    let ptr_b: *const i32 = &b;

    let result = unsafe { offset_from(ptr_a, ptr_b) };
    assert_eq!(result, (ptr_a as isize - ptr_b as isize) as usize);
}

#[test]
fn test_offset_from_same_memory_location() {
    let value = 30;
    let ptr_value: *const i32 = &value;

    let result = unsafe { offset_from(ptr_value, ptr_value) };
    assert_eq!(result, 0);
}

#[test]
fn test_offset_from_adjacent_memory_locations() {
    let arr = [1, 2, 3];

    let ptr_first: *const i32 = &arr[0];
    let ptr_second: *const i32 = &arr[1];

    let result = unsafe { offset_from(ptr_second, ptr_first) };
    assert_eq!(result, 1);
}

#[test]
fn test_offset_from_large_offsets() {
    let large_array: [i32; 100] = [0; 100];

    let ptr_start: *const i32 = &large_array[0];
    let ptr_end: *const i32 = &large_array[99];

    let result = unsafe { offset_from(ptr_end, ptr_start) };
    assert_eq!(result, 99);
}

