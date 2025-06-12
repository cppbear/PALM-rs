// Answer 0

#[test]
fn test_promotable_odd_clone_kind_vec() {
    let buf = vec![1u8, 2, 3, 4];
    let len = buf.len();
    let ptr = buf.as_ptr();
    
    let shared = Box::new(Shared {
        buf: buf.as_ptr() as *mut u8,
        cap: len,
        ref_cnt: AtomicUsize::new(1),
    });
    let shared = Box::into_raw(shared);
    
    let data = AtomicPtr::new(shared as _);
    
    unsafe {
        let result = promotable_odd_clone(&data, ptr, len);
    }
}

#[test]
fn test_promotable_odd_clone_non_null_ptr() {
    let buf = vec![5u8, 6, 7, 8];
    let len = buf.len();
    let ptr = buf.as_ptr();
    
    let shared = Box::new(Shared {
        buf: buf.as_ptr() as *mut u8,
        cap: len,
        ref_cnt: AtomicUsize::new(1),
    });
    let shared = Box::into_raw(shared);
    
    let data = AtomicPtr::new(shared as _);
    
    unsafe {
        let result = promotable_odd_clone(&data, ptr, len);
    }
}

#[test]
fn test_promotable_odd_clone_large_len() {
    let buf = vec![9u8; usize::MAX / 2];
    let len = buf.len();
    let ptr = buf.as_ptr();
    
    let shared = Box::new(Shared {
        buf: buf.as_ptr() as *mut u8,
        cap: len,
        ref_cnt: AtomicUsize::new(1),
    });
    let shared = Box::into_raw(shared);
    
    let data = AtomicPtr::new(shared as _);
    
    unsafe {
        let result = promotable_odd_clone(&data, ptr, len);
    }
}

#[test]
fn test_promotable_odd_clone_small_len() {
    let buf = vec![10u8];
    let len = buf.len();
    let ptr = buf.as_ptr();
    
    let shared = Box::new(Shared {
        buf: buf.as_ptr() as *mut u8,
        cap: len,
        ref_cnt: AtomicUsize::new(1),
    });
    let shared = Box::into_raw(shared);
    
    let data = AtomicPtr::new(shared as _);
    
    unsafe {
        let result = promotable_odd_clone(&data, ptr, len);
    }
}

#[test]
#[should_panic]
fn test_promotable_odd_clone_invalid_len() {
    let buf: Vec<u8> = vec![];
    let len = buf.len(); // len is zero
    let ptr = buf.as_ptr();
    
    let shared = Box::new(Shared {
        buf: buf.as_ptr() as *mut u8,
        cap: len,
        ref_cnt: AtomicUsize::new(1),
    });
    let shared = Box::into_raw(shared);
    
    let data = AtomicPtr::new(shared as _);
    
    unsafe {
        let result = promotable_odd_clone(&data, ptr, len);
    }
}

