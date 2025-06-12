// Answer 0

#[test]
fn test_deref_mut() {
    struct Shared {
        buf: *mut u8,
        cap: usize,
        ref_cnt: AtomicUsize,
    }

    let mut bytes_mut = BytesMut {
        ptr: NonNull::new(Box::into_raw(Box::new(0u8))).unwrap(),
        len: 1,
        cap: 1,
        data: Box::into_raw(Box::new(Shared {
            buf: Box::into_raw(Box::new([0u8; 1])),
            cap: 1,
            ref_cnt: AtomicUsize::new(1),
        })),
    };

    let slice: &mut [u8] = bytes_mut.deref_mut();
    slice[0] = 42;

    assert_eq!(slice[0], 42);

    unsafe {
        drop(Box::from_raw(bytes_mut.ptr.as_ptr()));
        drop(Box::from_raw((*bytes_mut.data).buf));
        drop(Box::from_raw(bytes_mut.data));
    }
}

#[test]
fn test_deref_mut_empty() {
    struct Shared {
        buf: *mut u8,
        cap: usize,
        ref_cnt: AtomicUsize,
    }

    let mut bytes_mut = BytesMut {
        ptr: NonNull::new(Box::into_raw(Box::new(0u8))).unwrap(),
        len: 0,
        cap: 1,
        data: Box::into_raw(Box::new(Shared {
            buf: Box::into_raw(Box::new([])),
            cap: 1,
            ref_cnt: AtomicUsize::new(1),
        })),
    };

    let slice: &mut [u8] = bytes_mut.deref_mut();

    assert_eq!(slice.len(), 0);

    unsafe {
        drop(Box::from_raw(bytes_mut.ptr.as_ptr()));
        drop(Box::from_raw((*bytes_mut.data).buf));
        drop(Box::from_raw(bytes_mut.data));
    }
}

