// Answer 0

#[test]
fn test_owned_is_unique() {
    use std::ptr::AtomicPtr;

    let ptr: AtomicPtr<()> = AtomicPtr::new(std::ptr::null_mut());
    unsafe {
        let result = owned_is_unique(&ptr);
        assert_eq!(result, false);
    }
}

