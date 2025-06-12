// Answer 0

#[test]
fn test_ptr_map_increments_address() {
    let value: usize = 1;
    let ptr = value as *mut u8;
    let increment = |x: usize| x + 1;
    let new_ptr = ptr_map(ptr, increment);
}

#[test]
fn test_ptr_map_multiplies_address() {
    let value: usize = 2;
    let ptr = value as *mut u8;
    let multiply = |x: usize| x * 2;
    let new_ptr = ptr_map(ptr, multiply);
}

#[test]
fn test_ptr_map_no_change() {
    let value: usize = 3;
    let ptr = value as *mut u8;
    let identity = |x: usize| x;
    let new_ptr = ptr_map(ptr, identity);
}

#[test]
fn test_ptr_map_large_address() {
    let value: usize = usize::MAX - 1;
    let ptr = value as *mut u8;
    let increment = |x: usize| x + 1;
    let new_ptr = ptr_map(ptr, increment);
}

#[test]
#[should_panic]
fn test_ptr_map_zero_address() {
    let ptr = 0 as *mut u8;
    let increment = |x: usize| x + 1;
    let new_ptr = ptr_map(ptr, increment);
}

#[test]
fn test_ptr_map_decrement_address() {
    let value: usize = 10;
    let ptr = value as *mut u8;
    let decrement = |x: usize| x - 1;
    let new_ptr = ptr_map(ptr, decrement);
}

