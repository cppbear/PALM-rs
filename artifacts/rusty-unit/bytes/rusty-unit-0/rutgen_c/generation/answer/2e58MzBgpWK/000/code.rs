// Answer 0

#[test]
fn test_promotable_even_clone_arc() {
    use core::ptr::null;

    let shared = Box::into_raw(Box::new(Shared {
        buf: null_mut(),
        cap: 0,
        ref_cnt: AtomicUsize::new(1),
    }));

    let data = AtomicPtr::new(shared as *mut _ | KIND_ARC as usize);
    let ptr = null();
    let len = 0;

    unsafe {
        let cloned = promotable_even_clone(&data, ptr, len);
        assert_eq!(cloned.len, len);
        assert_eq!(cloned.data.load(Ordering::Relaxed) as usize, shared as usize);
    }
}

#[test]
fn test_promotable_even_clone_vec() {
    use core::alloc::{alloc, dealloc, Layout};

    let layout = Layout::from_size_align(16, 8).unwrap();
    let buf = unsafe { alloc(layout) };
    let shared = Box::into_raw(Box::new(Shared {
        buf,
        cap: 16,
        ref_cnt: AtomicUsize::new(1),
    }));

    let data = AtomicPtr::new(shared as *mut _ | KIND_VEC as usize);
    let ptr = buf;  // Pointing to the allocated buffer
    let len = 16;

    unsafe {
        let cloned = promotable_even_clone(&data, ptr, len);
        assert_eq!(cloned.len, len);
        assert_eq!(cloned.data.load(Ordering::Relaxed) as usize, shared as usize);

        dealloc(buf, layout);
    }
}

