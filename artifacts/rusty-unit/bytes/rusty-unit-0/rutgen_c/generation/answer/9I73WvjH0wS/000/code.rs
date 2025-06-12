// Answer 0

#[test]
fn test_borrow_empty() {
    struct BytesMutTest {
        ptr: NonNull<u8>,
        len: usize,
        cap: usize,
        data: *mut Shared,
    }

    impl BytesMutTest {
        fn as_ref(&self) -> &[u8] {
            let slice = unsafe { slice::from_raw_parts(self.ptr.as_ptr(), self.len) };
            slice
        }

        fn borrow(&self) -> &[u8] {
            self.as_ref()
        }
    }

    let empty_vec: Vec<u8> = Vec::new();
    let ptr = NonNull::from(empty_vec.as_slice().as_ptr() as *mut u8);
    let bytes_mut = BytesMutTest {
        ptr,
        len: 0,
        cap: empty_vec.capacity(),
        data: std::ptr::null_mut(),
    };

    let borrowed = bytes_mut.borrow();
    assert_eq!(borrowed.len(), 0);
}

#[test]
fn test_borrow_non_empty() {
    struct BytesMutTest {
        ptr: NonNull<u8>,
        len: usize,
        cap: usize,
        data: *mut Shared,
    }

    impl BytesMutTest {
        fn as_ref(&self) -> &[u8] {
            let slice = unsafe { slice::from_raw_parts(self.ptr.as_ptr(), self.len) };
            slice
        }

        fn borrow(&self) -> &[u8] {
            self.as_ref()
        }
    }

    let data = vec![1, 2, 3, 4, 5];
    let ptr = NonNull::from(data.as_ptr() as *mut u8);
    let bytes_mut = BytesMutTest {
        ptr,
        len: data.len(),
        cap: data.capacity(),
        data: std::ptr::null_mut(),
    };

    let borrowed = bytes_mut.borrow();
    assert_eq!(borrowed, &[1, 2, 3, 4, 5]);
}

