// Answer 0

#[test]
fn test_owned_is_unique_case_1() {
    let atomic_ptr = AtomicPtr::new(ptr::null_mut());
    unsafe {
        owned_is_unique(&atomic_ptr);
    }
}

#[test]
fn test_owned_is_unique_case_2() {
    let atomic_ptr = AtomicPtr::new(ptr::null_mut());
    unsafe {
        owned_is_unique(&atomic_ptr);
    }
}

#[test]
fn test_owned_is_unique_case_3() {
    let atomic_ptr = AtomicPtr::new(ptr::null_mut());
    unsafe {
        owned_is_unique(&atomic_ptr);
    }
}

#[test]
fn test_owned_is_unique_edge_case() {
    let atomic_ptr = AtomicPtr::new(ptr::null_mut());
    unsafe {
        owned_is_unique(&atomic_ptr);
    }
}

