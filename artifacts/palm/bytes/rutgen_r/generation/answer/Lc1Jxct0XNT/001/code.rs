// Answer 0

#[test]
fn test_shared_v_to_mut_unique_case() {
    use std::ptr::null_mut;
    use std::sync::atomic::{AtomicPtr, Ordering};

    struct Shared {
        vec: Vec<u8>,
    }

    struct BytesMut {
        ptr: *mut u8,
        len: usize,
        cap: usize,
        data: *mut Shared,
    }

    // Unsafe function definition for testing
    unsafe fn shared_v_to_mut(data: &AtomicPtr<Shared>, ptr: *const u8, len: usize) -> BytesMut {
        let shared: *mut Shared = data.load(Ordering::Relaxed);
        if (*shared).is_unique() {
            let shared = &mut *shared;

            let v = &mut shared.vec;
            let v_capacity = v.capacity();
            let v_ptr = v.as_mut_ptr();
            let offset = offset_from(ptr as *mut u8, v_ptr);
            let cap = v_capacity - offset;

            let ptr = vptr(ptr as *mut u8);

            BytesMut {
                ptr,
                len,
                cap,
                data: shared,
            }
        } else {
            let v = slice::from_raw_parts(ptr, len).to_vec();
            release_shared(shared);
            BytesMut::from_vec(v)
        }
    }

    fn offset_from(ptr1: *mut u8, ptr2: *mut u8) -> usize {
        ptr1.offset_from(ptr2) as usize
    }

    fn vptr(ptr: *mut u8) -> *mut u8 {
        ptr
    }

    let vec = vec![1, 2, 3, 4, 5];
    let shared = Shared { vec: vec.clone() };
    let shared_ptr = Box::into_raw(Box::new(shared));
    let atomic_ptr = AtomicPtr::new(shared_ptr);
    
    let ptr = vec.as_ptr();
    let len = vec.len();

    unsafe {
        let bytes_mut = shared_v_to_mut(&atomic_ptr, ptr, len);
        assert_eq!(bytes_mut.len, len);
        assert_eq!(bytes_mut.cap, len);
        assert_eq!((*bytes_mut.data).vec, vec);
        assert_eq!(bytes_mut.ptr, ptr as *mut u8);
    }
}

