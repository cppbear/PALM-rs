// Answer 0

#[test]
fn test_set_vec_pos_valid() {
    struct BytesMutTest {
        kind: usize,
        data: *mut u8,
    }

    impl BytesMutTest {
        fn kind(&self) -> usize {
            self.kind
        }
    }

    const KIND_VEC: usize = 1;
    const MAX_VEC_POS: usize = 100;
    const VEC_POS_OFFSET: usize = 8;
    const NOT_VEC_POS_MASK: usize = !0xFF;

    unsafe fn invalid_ptr(ptr: usize) -> *mut u8 {
        ptr as *mut u8
    }

    let mut bytes_mut = BytesMutTest {
        kind: KIND_VEC,
        data: invalid_ptr(0),
    };

    unsafe {
        bytes_mut.set_vec_pos(50);
        assert_eq!(bytes_mut.data as usize, (50 << VEC_POS_OFFSET) | (0 as usize & NOT_VEC_POS_MASK));
    }
}

#[test]
#[should_panic]
fn test_set_vec_pos_exceeds_max() {
    struct BytesMutTest {
        kind: usize,
        data: *mut u8,
    }

    impl BytesMutTest {
        fn kind(&self) -> usize {
            self.kind
        }
    }

    const KIND_VEC: usize = 1;
    const MAX_VEC_POS: usize = 100;
    
    unsafe fn invalid_ptr(ptr: usize) -> *mut u8 {
        ptr as *mut u8
    }

    let mut bytes_mut = BytesMutTest {
        kind: KIND_VEC,
        data: invalid_ptr(0),
    };

    unsafe {
        bytes_mut.set_vec_pos(101);
    }
}

