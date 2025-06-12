// Answer 0

#[test]
#[should_panic]
fn test_shallow_clone_arc_above_max() {
    struct Shared {
        buf: *mut u8,
        cap: usize,
        ref_cnt: AtomicUsize,
    }
    
    let mut shared = Box::new(Shared {
        buf: std::ptr::null_mut(),
        cap: 0,
        ref_cnt: AtomicUsize::new(usize::MAX >> 1 + 1),
    });
    let shared_ptr = Box::into_raw(shared);
    let ptr = &10u8 as *const u8;
    let len = 1usize;

    unsafe {
        shallow_clone_arc(shared_ptr, ptr, len);
    }
}

#[test]
fn test_shallow_clone_arc_valid() {
    struct Shared {
        buf: *mut u8,
        cap: usize,
        ref_cnt: AtomicUsize,
    }
    
    let mut shared = Box::new(Shared {
        buf: std::ptr::null_mut(),
        cap: 0,
        ref_cnt: AtomicUsize::new(usize::MAX >> 1),
    });
    let shared_ptr = Box::into_raw(shared);
    let ptr = &20u8 as *const u8;
    let len = 1usize;

    unsafe {
        let _result = shallow_clone_arc(shared_ptr, ptr, len);
    }
}

#[test]
fn test_shallow_clone_arc_edge_case() {
    struct Shared {
        buf: *mut u8,
        cap: usize,
        ref_cnt: AtomicUsize,
    }
    
    let mut shared = Box::new(Shared {
        buf: std::ptr::null_mut(),
        cap: 0,
        ref_cnt: AtomicUsize::new(usize::MAX),
    });
    let shared_ptr = Box::into_raw(shared);
    let ptr = &30u8 as *const u8;
    let len = 1usize;

    unsafe {
        shallow_clone_arc(shared_ptr, ptr, len);
    }
}

#[test]
fn test_shallow_clone_arc_large_len() {
    struct Shared {
        buf: *mut u8,
        cap: usize,
        ref_cnt: AtomicUsize,
    }
    
    let mut shared = Box::new(Shared {
        buf: std::ptr::null_mut(),
        cap: 0,
        ref_cnt: AtomicUsize::new(usize::MAX >> 1),
    });
    let shared_ptr = Box::into_raw(shared);
    let ptr = &40u8 as *const u8;
    let len = usize::MAX;

    unsafe {
        let _result = shallow_clone_arc(shared_ptr, ptr, len);
    }
}

