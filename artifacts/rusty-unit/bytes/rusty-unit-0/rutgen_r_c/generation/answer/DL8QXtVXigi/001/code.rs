// Answer 0

#[test]
fn test_promotable_to_vec_with_kind_arc() {
    struct DataWrapper {
        atomic_ptr: AtomicPtr<()>,
    }

    impl DataWrapper {
        fn new() -> Self {
            let buf = Box::new(Shared {
                buf: Box::into_raw(Box::new([0u8; 10])),
                cap: 10,
                ref_cnt: AtomicUsize::new(1),
            });
            let atomic_ptr = AtomicPtr::new(Box::into_raw(buf) as *mut ());
            DataWrapper { atomic_ptr }
        }
    }

    let data_wrapper = DataWrapper::new();
    let ptr = data_wrapper.atomic_ptr.load(Ordering::Acquire) as *const u8;
    let len = 10;

    unsafe {
        let result = promotable_to_vec(
            &data_wrapper.atomic_ptr,
            ptr,
            len,
            |shared| {
                let shared_ref = shared as *mut Shared;
                (*shared_ref).buf
            },
        );

        assert_eq!(result.len(), len); // Verifying the length of the result Vec
        assert_eq!(result, vec![0; len]); // Verifying the content of the result Vec
    }
}

#[test]
#[should_panic]
fn test_promotable_to_vec_with_invalid_ptr() {
    struct DataWrapper {
        atomic_ptr: AtomicPtr<()>,
    }

    impl DataWrapper {
        fn new_invalid() -> Self {
            let atomic_ptr = AtomicPtr::new(ptr::null_mut());
            DataWrapper { atomic_ptr }
        }
    }

    let data_wrapper = DataWrapper::new_invalid();
    let ptr = ptr::null();
    let len = 10;

    unsafe {
        promotable_to_vec(
            &data_wrapper.atomic_ptr,
            ptr,
            len,
            |shared| {
                let shared_ref = shared as *mut Shared;
                (*shared_ref).buf
            },
        ); // This should panic
    }
}

