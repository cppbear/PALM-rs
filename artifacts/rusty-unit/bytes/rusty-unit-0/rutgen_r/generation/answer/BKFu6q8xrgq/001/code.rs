// Answer 0

#[test]
fn test_invalid_ptr_valid_address() {
    let addr: usize = 0x1000; // example valid address
    let ptr: *mut i32 = invalid_ptr::<i32>(addr);
    let value: i32 = 42; // example value to store at this address
    unsafe {
        ptr.write(value); // write value at the computed pointer
        assert_eq!(*ptr, value); // check if the value read back is the same
    }
}

#[test]
fn test_invalid_ptr_zero_address() {
    let addr: usize = 0; // zero address
    let ptr: *mut f64 = invalid_ptr::<f64>(addr);
    let value: f64 = 3.14; // example value to store at this address
    unsafe {
        ptr.write(value); // write value at the computed pointer
        assert_eq!(*ptr, value); // check if the value read back is the same
    }
}

#[test]
#[should_panic]
fn test_invalid_ptr_out_of_bounds() {
    let addr: usize = usize::MAX; // out of bounds address
    let _ptr: *mut i8 = invalid_ptr::<i8>(addr); // this should panic on assertion
}

#[test]
fn test_invalid_ptr_another_valid_address() {
    let addr: usize = 0x2000; // another example valid address
    let ptr: *mut u16 = invalid_ptr::<u16>(addr);
    let value: u16 = 123; // example value
    unsafe {
        ptr.write(value); // write value at the computed pointer
        assert_eq!(*ptr, value); // check if the value read back is the same
    }
}

