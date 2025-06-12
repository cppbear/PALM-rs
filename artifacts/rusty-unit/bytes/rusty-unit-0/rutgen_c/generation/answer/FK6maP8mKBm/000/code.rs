// Answer 0

#[test]
fn test_shared_to_vec_impl_unique() {
    use std::ptr;
    use std::alloc::{alloc, dealloc, Layout};
    use std::slice;

    let len = 10;
    let layout = Layout::array::<u8>(len).unwrap();
    let buf = unsafe { alloc(layout) };
    assert!(!buf.is_null());

    let shared = Box::into_raw(Box::new(Shared {
        buf,
        cap: len,
        ref_cnt: AtomicUsize::new(1),
    }));

    let data_ptr: *const u8 = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10].as_ptr();
    
    let vec = unsafe { shared_to_vec_impl(shared, data_ptr, len) };

    assert_eq!(vec.len(), len);
    assert_eq!(vec, vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);

    unsafe {
        dealloc(buf, layout);
    }
}

#[test]
fn test_shared_to_vec_impl_non_unique() {
    use std::ptr;
    use std::alloc::{alloc, dealloc, Layout};
    use std::slice;

    let len = 10;
    let layout = Layout::array::<u8>(len).unwrap();
    let buf = unsafe { alloc(layout) };
    assert!(!buf.is_null());

    let shared = Box::into_raw(Box::new(Shared {
        buf,
        cap: len,
        ref_cnt: AtomicUsize::new(2), // Make it non-unique
    }));

    let data_ptr: *const u8 = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10].as_ptr();
    
    let vec = unsafe { shared_to_vec_impl(shared, data_ptr, len) };

    assert_eq!(vec.len(), len);
    assert_eq!(vec, vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);

    unsafe {
        dealloc(buf, layout);
    }
}

#[should_panic]
#[test]
fn test_shared_to_vec_impl_invalid_pointer() {
    let invalid_shared: *mut Shared = std::ptr::null_mut();
    let data_ptr: *const u8 = std::ptr::null();
    
    unsafe {
        shared_to_vec_impl(invalid_shared, data_ptr, 0);
    }
}

