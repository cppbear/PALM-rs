// Answer 0

#[derive(Debug)]
struct Bytes {
    ptr: usize,
    len: usize,
    data: std::ptr::AtomicPtr<u8>,
    vtable: &'static VTable,
}

struct VTable;

fn without_provenance(addr: usize) -> usize {
    addr
}

const STATIC_VTABLE: VTable = VTable;

#[test]
fn test_new_empty_with_ptr_non_null() {
    let ptr: *const u8 = &0u8; // non-null pointer
    let bytes = new_empty_with_ptr(ptr);
    assert_eq!(bytes.len, 0);
    assert_eq!(bytes.ptr, without_provenance(ptr as usize));
}

#[test]
#[should_panic]
fn test_new_empty_with_ptr_null() {
    let ptr: *const u8 = std::ptr::null(); // null pointer
    new_empty_with_ptr(ptr); // this should trigger a panic
}

