// Answer 0

#[test]
fn test_promotable_to_mut_arc() {
    use std::sync::atomic::{AtomicPtr, Ordering};

    const KIND_MASK: usize = 0b11;
    const KIND_ARC: usize = 0b01;

    struct BytesMut {
        vec: Vec<u8>,
        offset: usize,
    }

    impl BytesMut {
        fn from_vec(vec: Vec<u8>) -> Self {
            Self { vec, offset: 0 }
        }

        fn advance_unchecked(&mut self, off: usize) {
            self.offset += off;
        }
    }

    unsafe fn shared_to_mut_impl(shared: *mut (), _ptr: *const u8, _len: usize) -> *mut u8 {
        shared as *mut u8
    }

    unsafe fn f(ptr: *mut ()) -> *mut u8 {
        ptr as *mut u8
    }

    let data = AtomicPtr::new(1 as *mut _);
    let result = unsafe {
        promotable_to_mut(
            &data,
            0 as *const u8,
            10,
            f,
        )
    };

    assert_eq!(result.vec.len(), 10);
}

#[test]
fn test_promotable_to_mut_vec() {
    use std::sync::atomic::{AtomicPtr, Ordering};

    const KIND_MASK: usize = 0b11;
    const KIND_VEC: usize = 0b10;

    struct BytesMut {
        vec: Vec<u8>,
        offset: usize,
    }

    impl BytesMut {
        fn from_vec(vec: Vec<u8>) -> Self {
            Self { vec, offset: 0 }
        }

        fn advance_unchecked(&mut self, off: usize) {
            self.offset += off;
        }
    }

    unsafe fn shared_to_mut_impl(shared: *mut (), _ptr: *const u8, _len: usize) -> *mut u8 {
        shared as *mut u8
    }

    unsafe fn f(ptr: *mut ()) -> *mut u8 {
        ptr as *mut u8
    }

    let data = AtomicPtr::new(2 as *mut _);
    let result = unsafe {
        promotable_to_mut(
            &data,
            0 as *const u8,
            10,
            f,
        )
    };

    assert_eq!(result.vec.len(), 10);
}

