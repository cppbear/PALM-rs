// Answer 0

#[test]
fn test_shared_to_mut_impl_only_handle() {
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::ptr::null_mut;
    use std::mem::ManuallyDrop;

    struct Shared {
        ref_cnt: AtomicUsize,
        buf: *mut u8,
        cap: usize,
    }

    struct BytesMut {
        vec: Vec<u8>,
    }

    impl BytesMut {
        fn from_vec(vec: Vec<u8>) -> Self {
            BytesMut { vec }
        }
        
        fn advance_unchecked(&mut self, offset: usize) {
            self.vec.drain(..offset);
        }
    }

    unsafe fn release_shared(shared: *mut Shared) {
        // Stub release function for test.
        drop(Box::from_raw(shared));
    }

    unsafe fn shared_to_mut_impl(shared: *mut Shared, ptr: *const u8, len: usize) -> BytesMut {
        if (*shared).ref_cnt.load(Ordering::Acquire) == 1 {
            let shared = *Box::from_raw(shared);
            let shared = ManuallyDrop::new(shared);
            let buf = shared.buf;
            let cap = shared.cap;
            let off = 0; // Assume offset is 0 for simplicity
            let v = Vec::from_raw_parts(buf, len + off, cap);
            let mut b = BytesMut::from_vec(v);
            b.advance_unchecked(off);
            b
        } else {
            let v = std::slice::from_raw_parts(ptr, len).to_vec();
            release_shared(shared);
            BytesMut::from_vec(v)
        }
    }

    unsafe {
        let buf = Box::into_raw(Box::new(42u8)) as *mut u8; // Simulating buffer allocation
        let shared = Box::into_raw(Box::new(Shared {
            ref_cnt: AtomicUsize::new(1),
            buf: buf,
            cap: 1,
        }));

        let result = shared_to_mut_impl(shared, buf, 1);
        assert_eq!(result.vec.len(), 1);
        assert_eq!(result.vec[0], 42);

        // Clean up
        release_shared(shared);
    }
}

