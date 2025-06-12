// Answer 0

#[test]
fn test_promotable_odd_clone_arc() {
    use core::ptr::null_mut;
    
    unsafe {
        let initial_shared = Box::new(crate::Shared {
            buf: null_mut(),
            cap: 0,
            ref_cnt: AtomicUsize::new(1),
        });
        
        let shared_ptr = Box::into_raw(initial_shared) as *mut ();
        let atomic_ptr = AtomicPtr::new(shared_ptr);
        
        let data = atomic_ptr;
        let ptr = b"Hello, world!" as *const u8;
        let len = 13;

        let bytes = promotable_odd_clone(&data, ptr, len);
        
        assert_eq!(bytes.len, len);
        assert_eq!(bytes.ptr, ptr);
        assert_eq!(bytes.data.load(Ordering::Relaxed) as *mut _, shared_ptr);
    }
}

#[test]
fn test_promotable_odd_clone_vec() {
    use core::ptr::null_mut;
    
    unsafe {
        let buf = Box::new([0u8; 20]);
        let initial_shared = Box::new(crate::Shared {
            buf: Box::into_raw(buf),
            cap: 20,
            ref_cnt: AtomicUsize::new(1),
        });
        
        let shared_ptr = Box::into_raw(initial_shared) as *mut ();
        let atomic_ptr = AtomicPtr::new(shared_ptr);
        
        let data = atomic_ptr;
        let ptr = b"Hello, world!" as *const u8;
        let len = 13;

        let bytes = promotable_odd_clone(&data, ptr, len);
        
        assert_eq!(bytes.len, len);
        assert_eq!(bytes.ptr, ptr);
        assert_eq!(bytes.data.load(Ordering::Relaxed) as *mut _, shared_ptr);
    }
}

