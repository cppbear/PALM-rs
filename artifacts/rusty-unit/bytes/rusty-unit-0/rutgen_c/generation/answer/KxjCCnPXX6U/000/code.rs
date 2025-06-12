// Answer 0

#[test]
fn test_borrow_mut() {
    struct TestBytesMut {
        ptr: NonNull<u8>,
        len: usize,
        cap: usize,
        data: *mut Shared,
    }

    impl TestBytesMut {
        fn new(vec: Vec<u8>) -> Self {
            let cap = vec.capacity();
            let ptr = NonNull::new(vec.as_mut_ptr()).unwrap();
            let len = vec.len();
            let shared = Box::into_raw(Box::new(Shared {
                vec,
                original_capacity_repr: 0,
                ref_count: AtomicUsize::new(1),
            }));
            TestBytesMut { ptr, len, cap, data: shared }
        }

        fn as_mut(&mut self) -> &mut [u8] {
            unsafe { slice::from_raw_parts_mut(self.ptr.as_ptr(), self.len) }
        }
    }

    let mut buf = TestBytesMut::new(vec![1, 2, 3, 4, 5]);
    let slice: &mut [u8] = buf.borrow_mut();
    slice[0] = 10;

    assert_eq!(slice, &[10, 2, 3, 4, 5]);
}

