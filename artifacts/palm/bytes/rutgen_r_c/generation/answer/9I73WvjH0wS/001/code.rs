// Answer 0

#[test]
fn test_borrow_returns_slice() {
    struct TestBytesMut {
        ptr: NonNull<u8>,
        len: usize,
        cap: usize,
        data: *mut Shared,
    }

    impl TestBytesMut {
        fn new(data: Vec<u8>) -> Self {
            let len = data.len();
            let cap = data.capacity();
            let ptr = NonNull::new(data.as_mut_ptr()).expect("Non-null pointer");
            let shared = Shared {
                buf: data.as_mut_ptr(),
                cap,
                ref_cnt: AtomicUsize::new(1),
            };
            let data_ptr = Box::into_raw(Box::new(shared));
            TestBytesMut {
                ptr,
                len,
                cap,
                data: data_ptr,
            }
        }

        fn as_ref(&self) -> &[u8] {
            unsafe {
                let shared = &*(self.data);
                let slice = slice::from_raw_parts(shared.buf, self.len);
                slice
            }
        }
    }

    let data = vec![1, 2, 3, 4, 5];
    let bytes_mut = TestBytesMut::new(data);
    let borrowed: &[u8] = bytes_mut.borrow();

    assert_eq!(borrowed, &[1, 2, 3, 4, 5]);
}

#[should_panic]
fn test_borrow_panics_on_empty() {
    struct TestBytesMut {
        ptr: NonNull<u8>,
        len: usize,
        cap: usize,
        data: *mut Shared,
    }

    impl TestBytesMut {
        fn new(data: Vec<u8>) -> Self {
            let len = data.len();
            let cap = data.capacity();
            let ptr = NonNull::new(data.as_mut_ptr()).expect("Non-null pointer");
            let shared = Shared {
                buf: data.as_mut_ptr(),
                cap,
                ref_cnt: AtomicUsize::new(1),
            };
            let data_ptr = Box::into_raw(Box::new(shared));
            TestBytesMut {
                ptr,
                len,
                cap,
                data: data_ptr,
            }
        }

        fn as_ref(&self) -> &[u8] {
            if self.len == 0 {
                panic!("Cannot borrow an empty slice");
            }
            unsafe {
                let shared = &*(self.data);
                let slice = slice::from_raw_parts(shared.buf, self.len);
                slice
            }
        }
    }

    let bytes_mut = TestBytesMut::new(vec![]);
    bytes_mut.borrow();
}

