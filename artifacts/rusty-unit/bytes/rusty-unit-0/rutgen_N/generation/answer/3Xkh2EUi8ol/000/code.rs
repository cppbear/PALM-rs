// Answer 0

#[test]
fn test_owned_clone() {
    use std::sync::atomic::{AtomicPtr, Ordering};
    use std::ptr;
    
    struct Bytes {
        ptr: *const u8,
        len: usize,
        data: AtomicPtr<()>,
        vtable: &'static VTable,
    }
    
    struct OwnedLifetime {
        ref_cnt: AtomicUsize,
    }
    
    struct VTable;
    
    const OWNED_VTABLE: VTable = VTable;
    
    unsafe fn owned_clone(data: &AtomicPtr<()>, ptr: *const u8, len: usize) -> Bytes {
        let owned = data.load(Ordering::Relaxed);
        let ref_cnt = &(*owned.cast::<OwnedLifetime>()).ref_cnt;
        let old_cnt = ref_cnt.fetch_add(1, Ordering::Relaxed);
        if old_cnt > usize::MAX >> 1 {
            crate::abort()
        }

        Bytes {
            ptr,
            len,
            data: AtomicPtr::new(owned as _),
            vtable: &OWNED_VTABLE,
        }
    }

    let owned_lifetime = OwnedLifetime {
        ref_cnt: AtomicUsize::new(1), // Initial reference count
    };
    let owned_ptr: AtomicPtr<()> = AtomicPtr::new(&owned_lifetime as *const _ as *mut ());

    let test_ptr: *const u8 = ptr::null();
    let test_len: usize = 0;

    let cloned_bytes = unsafe { owned_clone(&owned_ptr, test_ptr, test_len) };

    assert_eq!(cloned_bytes.len, test_len);
    assert_eq!(cloned_bytes.ptr, test_ptr);
}

