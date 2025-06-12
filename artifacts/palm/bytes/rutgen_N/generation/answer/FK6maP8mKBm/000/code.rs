// Answer 0

#[test]
fn test_shared_to_vec_impl_unique() {
    use std::alloc::{alloc, dealloc, Layout};
    use std::ptr::{self, NonNull};
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::mem::ManuallyDrop;

    struct Shared {
        ref_cnt: AtomicUsize,
        buf: *mut u8,
        cap: usize,
    }

    let len = 10;
    let layout = Layout::from_size_align(len, 1).unwrap();
    let buf = unsafe { alloc(layout) };
    for i in 0..len {
        unsafe { *buf.add(i) = i as u8 };
    }

    let shared = Box::into_raw(Box::new(Shared {
        ref_cnt: AtomicUsize::new(1),
        buf,
        cap: len,
    }));

    let ptr = unsafe { buf };
    
    let result = unsafe { shared_to_vec_impl(shared, ptr, len) };
    
    assert_eq!(result.len(), len);
    for i in 0..len {
        assert_eq!(result[i], i as u8);
    }

    unsafe {
        dealloc(buf, layout);
    }
}

#[test]
fn test_shared_to_vec_impl_non_unique() {
    use std::alloc::{alloc, dealloc, Layout};
    use std::ptr::{self, NonNull};
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::mem::ManuallyDrop;

    struct Shared {
        ref_cnt: AtomicUsize,
        buf: *mut u8,
        cap: usize,
    }

    let len = 10;
    let layout = Layout::from_size_align(len, 1).unwrap();
    let buf = unsafe { alloc(layout) };
    for i in 0..len {
        unsafe { *buf.add(i) = i as u8 };
    }

    let shared = Box::into_raw(Box::new(Shared {
        ref_cnt: AtomicUsize::new(2),
        buf,
        cap: len,
    }));

    let ptr = unsafe { buf };

    let result = unsafe { shared_to_vec_impl(shared, ptr, len) };

    assert_eq!(result.len(), len);
    for i in 0..len {
        assert_eq!(result[i], i as u8);
    }

    unsafe {
        dealloc(buf, layout);
    }
}

