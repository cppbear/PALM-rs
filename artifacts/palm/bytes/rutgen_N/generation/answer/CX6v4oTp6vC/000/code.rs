// Answer 0

#[test]
fn test_promotable_odd_clone_arc() {
    use std::sync::atomic::{AtomicPtr, Ordering};

    const KIND_MASK: usize = !0b11;
    const KIND_ARC: usize = 0b00; // example value for ARC kind
    const KIND_VEC: usize = 0b01; // example value for VEC kind

    struct Bytes {
        data: *const u8,
        len: usize,
    }

    unsafe fn shallow_clone_arc(shared: *const (), ptr: *const u8, len: usize) -> Bytes {
        // Mock implementation for the sake of the test
        Bytes { data: ptr, len }
    }

    let data: AtomicPtr<()> = AtomicPtr::new(KIND_ARC as *mut ());
    let ptr: *const u8 = b"example\0".as_ptr();
    let len: usize = 7;

    unsafe {
        let result = promotable_odd_clone(&data, ptr, len);
        assert_eq!(result.len, len);
        assert_eq!(result.data, ptr);
    }
}

#[test]
fn test_promotable_odd_clone_vec() {
    use std::sync::atomic::{AtomicPtr, Ordering};

    const KIND_MASK: usize = !0b11;
    const KIND_ARC: usize = 0b00; // example value for ARC kind
    const KIND_VEC: usize = 0b01; // example value for VEC kind

    struct Bytes {
        data: *const u8,
        len: usize,
    }

    unsafe fn shallow_clone_vec(data: &AtomicPtr<()>, shared: *const (), _origin: *const (), ptr: *const u8, len: usize) -> Bytes {
        // Mock implementation for the sake of the test
        Bytes { data: ptr, len }
    }

    let data: AtomicPtr<()> = AtomicPtr::new(KIND_VEC as *mut ());
    let ptr: *const u8 = b"example\0".as_ptr();
    let len: usize = 7;

    unsafe {
        let result = promotable_odd_clone(&data, ptr, len);
        assert_eq!(result.len, len);
        assert_eq!(result.data, ptr);
    }
}

