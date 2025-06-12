// Answer 0

#[test]
fn test_try_unsplit_with_empty_other() {
    struct BytesMut {
        ptr: *mut u8,
        len: usize,
        cap: usize,
        kind: u8,
        data: *const u8,
    }

    const KIND_ARC: u8 = 1;

    impl BytesMut {
        fn capacity(&self) -> usize {
            self.cap
        }

        fn kind(&self) -> u8 {
            self.kind
        }
    }

    let mut self_bytes = BytesMut {
        ptr: &mut 0 as *mut u8,
        len: 5,
        cap: 10,
        kind: KIND_ARC,
        data: &0 as *const u8,
    };

    let other_bytes = BytesMut {
        ptr: &mut 0 as *mut u8,
        len: 0,
        cap: 0,
        kind: KIND_ARC,
        data: &0 as *const u8,
    };

    let result = self_bytes.try_unsplit(other_bytes);
    assert_eq!(result, Ok(()));
}

#[test]
fn test_try_unsplit_with_contiguous_blocks() {
    struct BytesMut {
        ptr: *mut u8,
        len: usize,
        cap: usize,
        kind: u8,
        data: *const u8,
    }

    const KIND_ARC: u8 = 1;

    impl BytesMut {
        fn capacity(&self) -> usize {
            self.cap
        }

        fn kind(&self) -> u8 {
            self.kind
        }
    }

    let mut self_bytes = BytesMut {
        ptr: &mut 0 as *mut u8,
        len: 5,
        cap: 10,
        kind: KIND_ARC,
        data: &0 as *const u8,
    };

    let other_bytes = BytesMut {
        ptr: unsafe { self_bytes.ptr.add(self_bytes.len) }, // Simulates contiguous memory
        len: 5,
        cap: 5,
        kind: KIND_ARC,
        data: self_bytes.data,
    };

    let result = self_bytes.try_unsplit(other_bytes);
    assert_eq!(result, Ok(()));
    assert_eq!(self_bytes.len, 10);
    assert_eq!(self_bytes.cap, 15);
}

