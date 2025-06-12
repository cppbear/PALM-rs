// Answer 0

#[test]
fn test_static_is_unique_null_pointer() {
    let ptr = AtomicPtr::new(std::ptr::null_mut());
    static_is_unique(&ptr);
}

#[test]
fn test_static_is_unique_minimum_pointer() {
    let ptr = AtomicPtr::new(NonNull::new_unchecked(0x1 as *mut ()));
    static_is_unique(&ptr);
}

#[test]
fn test_static_is_unique_mid_pointer() {
    let ptr = AtomicPtr::new(NonNull::new_unchecked(0x7fffffff as *mut ()));
    static_is_unique(&ptr);
}

#[test]
fn test_static_is_unique_maximum_pointer() {
    let ptr = AtomicPtr::new(NonNull::new_unchecked(usize::MAX as *mut ()));
    static_is_unique(&ptr);
}

#[test]
fn test_static_is_unique_large_aligned_pointer() {
    let ptr = AtomicPtr::new(NonNull::new_unchecked(0x100000000 as *mut ()));
    static_is_unique(&ptr);
}

