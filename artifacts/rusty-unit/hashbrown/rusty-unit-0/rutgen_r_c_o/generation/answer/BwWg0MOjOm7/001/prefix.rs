// Answer 0

#[test]
fn test_offset_from_valid_pointers() {
    let x: i32 = 10;
    let y: i32 = 20;

    let from: *const i32 = &x;
    let to: *const i32 = &y;

    unsafe {
        let result = offset_from(to, from);
    }
}

#[test]
fn test_offset_from_adjacent_pointers() {
    let array: [i32; 3] = [1, 2, 3];
    
    let from: *const i32 = &array[0];
    let to: *const i32 = &array[1];

    unsafe {
        let result = offset_from(to, from);
    }
}

#[test]
fn test_offset_from_same_pointer() {
    let value: i32 = 42;
    
    let ptr: *const i32 = &value;

    unsafe {
        let result = offset_from(ptr, ptr);
    }
}

#[test]
fn test_offset_from_large_pointer_difference() {
    let a: [i32; 5] = [0, 1, 2, 3, 4];
    let b: [i32; 5] = [5, 6, 7, 8, 9];

    let from: *const i32 = &a[0];
    let to: *const i32 = &b[0];

    unsafe {
        let result = offset_from(to, from);
    }
}

#[test]
#[should_panic]
fn test_offset_from_invalid_pointer() {
    let invalid_ptr: *const i32 = core::ptr::null();
    let valid_ptr: *const i32 = &42;

    unsafe {
        let result = offset_from(valid_ptr, invalid_ptr);
    }
}

