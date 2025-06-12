// Answer 0

#[test]
#[should_panic]
fn test_new_empty_with_null_ptr() {
    let null_ptr: *const u8 = std::ptr::null();
    let _bytes = new_empty_with_ptr(null_ptr);
}

#[test]
fn test_new_empty_with_non_null_ptr() {
    let non_null_ptr: *const u8 = &0;
    let bytes = new_empty_with_ptr(non_null_ptr);
    
    assert_eq!(bytes.len, 0);
    assert!(bytes.ptr != 0);
}

struct Bytes {
    ptr: usize,
    len: usize,
    data: std::sync::atomic::AtomicPtr<u8>,
    vtable: &'static str,
}

static STATIC_VTABLE: &str = "static_vtable";

fn without_provenance(addr: usize) -> usize {
    addr
}

