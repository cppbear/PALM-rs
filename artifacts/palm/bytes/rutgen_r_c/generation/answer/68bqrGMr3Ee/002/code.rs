// Answer 0

fn test_shallow_clone_arc_valid() {
    use core::sync::atomic::AtomicUsize;
    
    struct Shared {
        buf: *mut u8,
        cap: usize,
        ref_cnt: AtomicUsize,
    }

    unsafe {
        let shared_instance = Shared {
            buf: core::ptr::null_mut(),
            cap: 1024,
            ref_cnt: AtomicUsize::new(usize::MAX >> 1),
        };

        let shared_ptr = &shared_instance as *const Shared as *mut Shared;
        let ptr: *const u8 = core::ptr::null();
        let len: usize = 0;

        let result = shallow_clone_arc(shared_ptr, ptr, len);

        assert_eq!(result.len, len);
        assert_eq!(result.ptr, ptr);
        assert_eq!(result.data.load(Ordering::Relaxed), shared_ptr as *mut ());
        assert_eq!(result.vtable, &SHARED_VTABLE);
        assert_eq!(shared_instance.ref_cnt.load(Ordering::Relaxed), usize::MAX >> 1 + 1);
    }
}

#[should_panic]
fn test_shallow_clone_arc_panic() {
    use core::sync::atomic::AtomicUsize;

    struct Shared {
        buf: *mut u8,
        cap: usize,
        ref_cnt: AtomicUsize,
    }

    unsafe {
        let shared_instance = Shared {
            buf: core::ptr::null_mut(),
            cap: 1024,
            ref_cnt: AtomicUsize::new(usize::MAX >> 1 + 1),
        };

        let shared_ptr = &shared_instance as *const Shared as *mut Shared;
        let ptr: *const u8 = core::ptr::null();
        let len: usize = 0;

        shallow_clone_arc(shared_ptr, ptr, len);
    }
}

