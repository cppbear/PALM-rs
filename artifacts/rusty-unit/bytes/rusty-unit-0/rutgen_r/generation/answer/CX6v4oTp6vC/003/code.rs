// Answer 0

#[test]
fn test_promotable_odd_clone_vec_case() {
    use std::sync::atomic::{AtomicPtr, Ordering};

    const KIND_MASK: usize = 0b11;
    const KIND_ARC: usize = 0b01;
    const KIND_VEC: usize = 0b10;

    struct Bytes {
        _data: *const u8,
        _len: usize,
    }

    // Dummy implementations of shallow_clone_arc and shallow_clone_vec
    unsafe fn shallow_clone_arc(_shared: *const (), _ptr: *const u8, _len: usize) -> Bytes {
        Bytes { _data: _ptr, _len }
    }

    unsafe fn shallow_clone_vec(_data: &AtomicPtr<()>, _shared: *const (), _raw_shared: *const (), _ptr: *const u8, _len: usize) -> Bytes {
        Bytes { _data: _ptr, _len }
    }

    let data = AtomicPtr::new(Box::into_raw(Box::new(0u8 as usize | KIND_VEC)) as *mut ());
    let ptr = [1u8, 2u8, 3u8].as_ptr();
    let len = 3;

    let result = unsafe { promotable_odd_clone(&data, ptr, len) };
    
    assert_eq!(result._len, len);
    assert_eq!(result._data, ptr);

    // Clean up, if necessary (not causing a memory leak)
    let _ = unsafe { Box::from_raw(data.load(Ordering::Acquire) as *mut usize) }; 
}

#[test]
#[should_panic]
fn test_promotable_odd_clone_arc_case() {
    use std::sync::atomic::{AtomicPtr, Ordering};

    const KIND_MASK: usize = 0b11;
    const KIND_ARC: usize = 0b01;

    struct Bytes {
        _data: *const u8,
        _len: usize,
    }

    // Dummy implementations of shallow_clone_arc and shallow_clone_vec
    unsafe fn shallow_clone_arc(_shared: *const (), _ptr: *const u8, _len: usize) -> Bytes {
        Bytes { _data: _ptr, _len }
    }

    unsafe fn shallow_clone_vec(_data: &AtomicPtr<()>, _shared: *const (), _raw_shared: *const (), _ptr: *const u8, _len: usize) -> Bytes {
        Bytes { _data: _ptr, _len }
    }

    let data = AtomicPtr::new(Box::into_raw(Box::new(0u8 as usize | KIND_ARC)) as *mut ());
    let ptr = [1u8, 2u8, 3u8].as_ptr();
    let len = 3;

    let _ = unsafe { promotable_odd_clone(&data, ptr, len) };
}

