// Answer 0

#[test]
fn test_shared_v_to_mut_unique_case() {
    use core::ptr::NonNull;
    use alloc::vec::Vec;
    use core::sync::atomic::{AtomicPtr, Ordering};
    
    struct Shared {
        vec: Vec<u8>,
        ref_count: AtomicUsize,
    }

    unsafe fn create_shared() -> (AtomicPtr<()>, NonNull<u8>, usize) {
        let vec = Vec::from_iter(vec![1, 2, 3, 4, 5]);
        let shared = Box::into_raw(Box::new(Shared {
            vec,
            ref_count: AtomicUsize::new(1),
        }));
        let atom_ptr = AtomicPtr::new(shared as *mut ());
        let ptr = NonNull::new_unchecked((*shared).vec.as_mut_ptr());
        let len = (*shared).vec.len();
        (atom_ptr, ptr, len)
    }

    let (data, ptr, len) = unsafe { create_shared() };

    let bytes_mut = unsafe { shared_v_to_mut(&data, ptr.as_ptr(), len) };

    assert_eq!(bytes_mut.len(), len);
    assert_eq!(bytes_mut.capacity(), len);
    assert_eq!(bytes_mut.data, *(data.load(Ordering::Relaxed) as *mut Shared));
} 

#[test]
fn test_shared_v_to_mut_non_unique_case() {
    use core::ptr::NonNull;
    use alloc::vec::Vec;
    use core::sync::atomic::{AtomicPtr, Ordering};
    
    struct Shared {
        vec: Vec<u8>,
        ref_count: AtomicUsize,
    }

    unsafe fn create_non_unique_shared() -> (AtomicPtr<()>, NonNull<u8>, usize) {
        let vec = Vec::from_iter(vec![1, 2, 3, 4, 5]);
        let shared = Box::into_raw(Box::new(Shared {
            vec,
            ref_count: AtomicUsize::new(2),
        }));
        let atom_ptr = AtomicPtr::new(shared as *mut ());
        let ptr = NonNull::new_unchecked((*shared).vec.as_mut_ptr());
        let len = (*shared).vec.len();
        (atom_ptr, ptr, len)
    }

    let (data, ptr, len) = unsafe { create_non_unique_shared() };

    let bytes_mut = unsafe { shared_v_to_mut(&data, ptr.as_ptr(), len) };

    assert_eq!(bytes_mut.len(), len);
    assert_eq!(bytes_mut.capacity(), len);
}

