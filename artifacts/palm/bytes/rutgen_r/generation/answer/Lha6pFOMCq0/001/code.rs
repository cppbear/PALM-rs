// Answer 0

#[test]
fn test_try_unsplit_contiguous_blocks() {
    struct BytesMut {
        ptr: *mut u8,
        len: usize,
        cap: usize,
        kind: usize,
        data: usize,
    }

    const KIND_ARC: usize = 1;

    impl BytesMut {
        fn new(len: usize, cap: usize, kind: usize, data: usize) -> Self {
            let ptr = Box::into_raw(vec![0u8; cap].into_boxed_slice()) as *mut u8;
            BytesMut { ptr, len, cap, kind, data }
        }

        fn capacity(&self) -> usize {
            self.cap
        }

        fn kind(&self) -> usize {
            self.kind
        }

        fn try_unsplit(&mut self, other: BytesMut) -> Result<(), BytesMut> {
            if other.capacity() == 0 {
                return Ok(());
            }

            let ptr = unsafe { self.ptr.add(self.len) };
            if ptr == other.ptr
                && self.kind() == KIND_ARC
                && other.kind() == KIND_ARC
                && self.data == other.data
            {
                self.len += other.len;
                self.cap += other.cap;
                Ok(())
            } else {
                Err(other)
            }
        }
    }

    let mut bytes1 = BytesMut::new(5, 10, KIND_ARC, 42);
    let bytes2 = BytesMut::new(5, 10, KIND_ARC, 42);

    let result = bytes1.try_unsplit(bytes2);
    assert!(result.is_ok());
}

