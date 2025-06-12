// Answer 0

#[derive(Debug)]
struct BytesMut {
    ptr: *mut u8,
    len: usize,
    cap: usize,
    kind: u8,
}

const KIND_ARC: u8 = 1;

impl BytesMut {
    fn new(len: usize, cap: usize, kind: u8) -> Self {
        let ptr = unsafe {
            let layout = std::alloc::Layout::from_size_align(cap, 1).unwrap();
            std::alloc::alloc(layout)
        };
        BytesMut { ptr, len, cap, kind }
    }

    fn capacity(&self) -> usize {
        self.cap
    }

    fn kind(&self) -> u8 {
        self.kind
    }
}

#[test]
fn test_try_unsplit_with_zero_capacity() {
    let mut self_bytes = BytesMut::new(5, 10, KIND_ARC);
    let other_bytes = BytesMut::new(0, 0, 2); // other.kind() != KIND_ARC

    let result = self_bytes.try_unsplit(other_bytes);
    
    assert!(result.is_err());
}

#[test]
fn test_try_unsplit_with_same_ptr_and_different_kinds() {
    let mut self_bytes = BytesMut::new(5, 10, KIND_ARC);
    let other_bytes = BytesMut::new(5, 10, 2); // other.kind() != KIND_ARC

    // Manually moving pointers to simulate contiguous memory (unsafe)
    unsafe {
        std::ptr::copy_nonoverlapping(
            self_bytes.ptr,
            self_bytes.ptr.add(self_bytes.len),
            other_bytes.len,
        );
    }
    let result = self_bytes.try_unsplit(other_bytes);

    assert!(result.is_err());
}

