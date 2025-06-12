// Answer 0

#[test]
fn test_kind_vec() {
    struct Shared {
        buf: *mut u8,
        cap: usize,
        ref_cnt: AtomicUsize,
    }

    let shared = Shared {
        buf: std::ptr::null_mut(),
        cap: 0,
        ref_cnt: AtomicUsize::new(1),
    };

    let bytes_mut = BytesMut {
        ptr: NonNull::new(std::ptr::null_mut()).unwrap(),
        len: 0,
        cap: 0,
        data: &shared as *const _ as *mut _,
    };

    assert_eq!(bytes_mut.kind(), KIND_VEC);
}

#[test]
fn test_kind_arc() {
    struct Shared {
        vec: Vec<u8>,
        original_capacity_repr: usize,
        ref_count: AtomicUsize,
    }

    let vec = Vec::new();
    let shared = Shared {
        vec,
        original_capacity_repr: 0,
        ref_count: AtomicUsize::new(1),
    };

    let bytes_mut = BytesMut {
        ptr: NonNull::new(std::ptr::null_mut()).unwrap(),
        len: 0,
        cap: 0,
        data: &shared as *const _ as *mut _,
    };

    assert_eq!(bytes_mut.kind(), KIND_ARC);
}

