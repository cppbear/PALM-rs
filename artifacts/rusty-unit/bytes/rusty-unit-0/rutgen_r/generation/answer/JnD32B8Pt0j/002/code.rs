// Answer 0

#[test]
fn test_set_vec_pos_with_valid_position() {
    const KIND_VEC: u32 = 1;
    const MAX_VEC_POS: usize = 100;
    const VEC_POS_OFFSET: usize = 10;
    const NOT_VEC_POS_MASK: usize = !0b1111111111; // Assuming valid bits are lower 10 bits

    struct BytesMut {
        kind: u32,
        data: *mut u8,
    }

    impl BytesMut {
        fn kind(&self) -> u32 {
            self.kind
        }

        unsafe fn set_vec_pos(&mut self, pos: usize) {
            debug_assert_eq!(self.kind(), KIND_VEC);
            debug_assert!(pos <= MAX_VEC_POS);

            self.data = invalid_ptr((pos << VEC_POS_OFFSET) | (self.data as usize & NOT_VEC_POS_MASK));
        }
    }

    fn invalid_ptr(val: usize) -> *mut u8 {
        val as *mut u8
    }

    let mut bytes = BytesMut {
        kind: KIND_VEC,
        data: invalid_ptr(0),
    };

    unsafe {
        bytes.set_vec_pos(50); // Valid position, should not panic
    }
}

#[test]
#[should_panic]
fn test_set_vec_pos_with_exceeding_position() {
    const KIND_VEC: u32 = 1;
    const MAX_VEC_POS: usize = 100;
    const VEC_POS_OFFSET: usize = 10;
    const NOT_VEC_POS_MASK: usize = !0b1111111111; // Assuming valid bits are lower 10 bits

    struct BytesMut {
        kind: u32,
        data: *mut u8,
    }

    impl BytesMut {
        fn kind(&self) -> u32 {
            self.kind
        }

        unsafe fn set_vec_pos(&mut self, pos: usize) {
            debug_assert_eq!(self.kind(), KIND_VEC);
            debug_assert!(pos <= MAX_VEC_POS);

            self.data = invalid_ptr((pos << VEC_POS_OFFSET) | (self.data as usize & NOT_VEC_POS_MASK));
        }
    }

    fn invalid_ptr(val: usize) -> *mut u8 {
        val as *mut u8
    }

    let mut bytes = BytesMut {
        kind: KIND_VEC,
        data: invalid_ptr(0),
    };

    unsafe {
        bytes.set_vec_pos(101); // Exceeding position, should panic
    }
}

