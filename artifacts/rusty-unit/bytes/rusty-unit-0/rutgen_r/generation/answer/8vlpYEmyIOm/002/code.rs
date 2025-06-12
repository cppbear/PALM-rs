// Answer 0

#[test]
fn test_new_empty_with_ptr_valid_pointer() {
    use std::ptr::null;

    struct Bytes {
        ptr: usize,
        len: usize,
        data: std::sync::atomic::AtomicPtr<u8>,
        vtable: &'static str,
    }

    const STATIC_VTABLE: &str = "static_vtable";

    fn without_provenance(ptr: usize) -> usize {
        ptr // In this test, we simply return the original ptr for simplicity
    }

    fn new_empty_with_ptr(ptr: *const u8) -> Bytes {
        debug_assert!(!ptr.is_null());

        let ptr = without_provenance(ptr as usize);

        Bytes {
            ptr,
            len: 0,
            data: std::sync::atomic::AtomicPtr::new(ptr::null_mut()),
            vtable: STATIC_VTABLE,
        }
    }

    let valid_ptr = &0u8 as *const u8; // A valid pointer to a u8
    let bytes = new_empty_with_ptr(valid_ptr);

    assert_eq!(bytes.ptr, without_provenance(valid_ptr as usize));
    assert_eq!(bytes.len, 0);
    assert_eq!(bytes.data.load(std::sync::atomic::Ordering::SeqCst), null_mut());
    assert_eq!(bytes.vtable, STATIC_VTABLE);
}

#[should_panic]
#[test]
fn test_new_empty_with_ptr_null_pointer() {
    fn new_empty_with_ptr(ptr: *const u8) -> ! {
        debug_assert!(!ptr.is_null());
        panic!("This should never be called with a null pointer");
    }

    new_empty_with_ptr(std::ptr::null());
}

