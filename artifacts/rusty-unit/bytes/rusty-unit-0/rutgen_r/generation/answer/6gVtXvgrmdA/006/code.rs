// Answer 0

#[test]
fn test_promote_to_shared_valid_ref_count_one() {
    struct MyBytesMut {
        data: *mut u8,
        len: usize,
        cap: usize,
    }

    struct Shared {
        vec: Vec<u8>,
        original_capacity_repr: usize,
        ref_count: std::sync::atomic::AtomicUsize,
    }

    unsafe fn rebuild_vec(ptr: *const u8, len: usize, cap: usize, offset: usize) -> Vec<u8> {
        Vec::from_raw_parts(ptr as *mut u8, len, cap)
    }

    const KIND_VEC: usize = 1;
    const KIND_MASK: usize = 0b11;
    const KIND_ARC: usize = 2;
    const ORIGINAL_CAPACITY_MASK: usize = 0xFFFF;
    const ORIGINAL_CAPACITY_OFFSET: usize = 16;
    const VEC_POS_OFFSET: usize = 8;

    let mut my_bytes = MyBytesMut {
        data: &mut 0u8 as *mut _,
        len: 10,
        cap: 20,
    };

    my_bytes.data = (&my_bytes.data as *const _ as usize | KIND_VEC as usize) as *mut u8;
    let ref_cnt = 1;

    unsafe {
        my_bytes.promote_to_shared(ref_cnt);
    }

    assert!(my_bytes.data as usize & KIND_MASK == KIND_ARC);
}

#[test]
fn test_promote_to_shared_valid_ref_count_two() {
    struct MyBytesMut {
        data: *mut u8,
        len: usize,
        cap: usize,
    }

    struct Shared {
        vec: Vec<u8>,
        original_capacity_repr: usize,
        ref_count: std::sync::atomic::AtomicUsize,
    }

    unsafe fn rebuild_vec(ptr: *const u8, len: usize, cap: usize, offset: usize) -> Vec<u8> {
        Vec::from_raw_parts(ptr as *mut u8, len, cap)
    }

    const KIND_VEC: usize = 1;
    const KIND_MASK: usize = 0b11;
    const KIND_ARC: usize = 2;
    const ORIGINAL_CAPACITY_MASK: usize = 0xFFFF;
    const ORIGINAL_CAPACITY_OFFSET: usize = 16;
    const VEC_POS_OFFSET: usize = 8;

    let mut my_bytes = MyBytesMut {
        data: &mut 0u8 as *mut _,
        len: 5,
        cap: 10,
    };

    my_bytes.data = (&my_bytes.data as *const _ as usize | KIND_VEC as usize) as *mut u8;
    let ref_cnt = 2;

    unsafe {
        my_bytes.promote_to_shared(ref_cnt);
    }

    assert!(my_bytes.data as usize & KIND_MASK == KIND_ARC);
}

