// Answer 0

#[test]
fn test_kind_with_kind_arc() {
    struct TestBytesMut {
        data: *mut Shared,
    }

    let shared = Shared {
        buf: std::ptr::null_mut(),
        cap: 0,
        ref_cnt: AtomicUsize::new(0),
    };
    
    let test_bytes_mut = TestBytesMut {
        data: &shared as *const _ as *mut _,
    };

    let result = (test_bytes_mut.data as usize) & KIND_MASK;
    assert_eq!(result, KIND_ARC);
}

#[test]
fn test_kind_with_kind_vec() {
    struct TestBytesMut {
        data: *mut Shared,
    }

    let shared = Shared {
        vec: Vec::new(),
        original_capacity_repr: 0,
        ref_count: AtomicUsize::new(0),
    };
    
    let test_bytes_mut = TestBytesMut {
        data: &shared as *const _ as *mut _,
    };

    let result = (test_bytes_mut.data as usize) & KIND_MASK;
    assert_eq!(result, KIND_VEC);
}

#[test]
fn test_kind_with_null_data() {
    struct TestBytesMut {
        data: *mut Shared,
    }

    let test_bytes_mut = TestBytesMut {
        data: std::ptr::null_mut(),
    };

    let result = (test_bytes_mut.data as usize) & KIND_MASK;
    assert_eq!(result, 0);
}

