// Answer 0

#[test]
fn test_shallow_clone_arc() {
    use std::ptr;

    struct BytesMut {
        data: *mut u8,
        ref_count: usize,
        kind: usize,
    }

    const KIND_ARC: usize = 1;

    impl BytesMut {
        fn kind(&self) -> usize {
            self.kind
        }

        unsafe fn shallow_clone(&mut self) -> BytesMut {
            if self.kind() == KIND_ARC {
                increment_shared(self.data);
                ptr::read(self)
            } else {
                self.promote_to_shared(2);
                ptr::read(self)
            }
        }

        unsafe fn promote_to_shared(&mut self, ref_count: usize) {
            self.ref_count = ref_count;
        }
    }
    
    unsafe fn increment_shared(data: *mut u8) {
        // Simulate incrementing a shared reference count.
    }

    let mut original = BytesMut {
        data: ptr::null_mut(),
        ref_count: 1,
        kind: KIND_ARC,
    };

    let cloned = unsafe { original.shallow_clone() };

    assert_eq!(original.ref_count, 1);
    assert_eq!(cloned.ref_count, 1);
    assert_eq!(original.kind(), KIND_ARC);
}

#[test]
fn test_shallow_clone_non_arc() {
    use std::ptr;

    struct BytesMut {
        data: *mut u8,
        ref_count: usize,
        kind: usize,
    }

    const KIND_OTHER: usize = 0;

    impl BytesMut {
        fn kind(&self) -> usize {
            self.kind
        }

        unsafe fn shallow_clone(&mut self) -> BytesMut {
            if self.kind() == KIND_ARC {
                increment_shared(self.data);
                ptr::read(self)
            } else {
                self.promote_to_shared(2);
                ptr::read(self)
            }
        }

        unsafe fn promote_to_shared(&mut self, ref_count: usize) {
            self.ref_count = ref_count;
        }
    }
    
    unsafe fn increment_shared(data: *mut u8) {
        // Simulate incrementing a shared reference count.
    }

    let mut original = BytesMut {
        data: ptr::null_mut(),
        ref_count: 1,
        kind: KIND_OTHER,
    };

    let cloned = unsafe { original.shallow_clone() };

    assert_eq!(original.ref_count, 2);
    assert_eq!(cloned.ref_count, 2);
    assert_eq!(original.kind(), KIND_OTHER);
}

