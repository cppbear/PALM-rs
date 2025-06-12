// Answer 0

#[test]
fn test_shared_to_mut_unique_reference() {
    use core::ptr;

    struct DummyShared {
        buf: *mut u8,
        cap: usize,
        ref_cnt: AtomicUsize,
    }

    unsafe impl Send for DummyShared {}
    unsafe impl Sync for DummyShared {}

    let buf = Box::into_raw(Box::new([1, 2, 3, 4, 5])) as *mut u8;
    let shared = Box::into_raw(Box::new(DummyShared {
        buf,
        cap: 5,
        ref_cnt: AtomicUsize::new(1),
    }));

    let data = AtomicPtr::new(shared.cast());
    let ptr = buf;
    let len = 5;

    let bytes_mut = unsafe { shared_to_mut(&data, ptr, len) };

    assert_eq!(bytes_mut.len, 5);
    assert_eq!(bytes_mut.cap, 5);

    let slice = unsafe { slice::from_raw_parts(bytes_mut.ptr.as_ptr(), bytes_mut.len) };
    assert_eq!(slice, &[1, 2, 3, 4, 5]);

    // Cleanup
    unsafe {
        dealloc(ptr, Layout::array::<u8>(5).unwrap());
        drop(Box::from_raw(shared));
    }
}

#[test]
fn test_shared_to_mut_multiple_references() {
    use core::ptr;

    struct DummyShared {
        buf: *mut u8,
        cap: usize,
        ref_cnt: AtomicUsize,
    }

    unsafe impl Send for DummyShared {}
    unsafe impl Sync for DummyShared {}

    let buf = Box::into_raw(Box::new([6, 7, 8, 9, 10])) as *mut u8;
    let shared = Box::into_raw(Box::new(DummyShared {
        buf,
        cap: 5,
        ref_cnt: AtomicUsize::new(2),
    }));

    let data = AtomicPtr::new(shared.cast());
    let ptr = buf;
    let len = 5;

    let bytes_mut = unsafe { shared_to_mut(&data, ptr, len) };

    assert_eq!(bytes_mut.len, 5);
    assert_eq!(bytes_mut.cap, 5);

    let slice = unsafe { slice::from_raw_parts(bytes_mut.ptr.as_ptr(), bytes_mut.len) };
    assert_eq!(slice, &[6, 7, 8, 9, 10]);

    // Cleanup
    unsafe {
        dealloc(ptr, Layout::array::<u8>(5).unwrap());
        drop(Box::from_raw(shared));
    }
}

