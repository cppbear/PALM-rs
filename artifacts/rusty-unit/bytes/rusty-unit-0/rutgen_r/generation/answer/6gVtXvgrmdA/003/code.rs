// Answer 0

#[test]
fn test_promote_to_shared_with_ref_cnt_1() {
    use std::sync::atomic::AtomicUsize;

    struct Shared {
        vec: Vec<u8>,
        original_capacity_repr: usize,
        ref_count: AtomicUsize,
    }

    struct BytesMut {
        data: *mut Shared,
        len: usize,
        cap: usize,
    }

    const KIND_VEC: usize = 0;
    const KIND_ARC: usize = 1;
    const ORIGINAL_CAPACITY_MASK: usize = 0xFFFF;
    const ORIGINAL_CAPACITY_OFFSET: usize = 16;
    const VEC_POS_OFFSET: usize = 32;
    
    unsafe fn rebuild_vec(ptr: *const u8, len: usize, cap: usize, off: usize) -> Vec<u8> {
        Vec::from_raw_parts(ptr as *mut u8, len, cap)
    }

    let vec = vec![1, 2, 3];
    let len = vec.len();
    let cap = vec.capacity();
    let original_capacity_repr = 0; // Assuming original capacity representation for testing.

    let mut byte_mut = BytesMut {
        data: std::ptr::null_mut(),
        len,
        cap,
    };

    // Initialize with KIND_VEC
    byte_mut.data = Box::into_raw(Box::new(Shared {
        vec: vec,
        original_capacity_repr,
        ref_count: AtomicUsize::new(1), // ref_cnt set to 1
    }));

    unsafe {
        byte_mut.promote_to_shared(1); // Testing with ref_cnt == 1

        let shared = byte_mut.data;
        assert!(!shared.is_null());
        assert_eq!((*shared).ref_count.load(std::sync::atomic::Ordering::SeqCst), 1);
        assert_eq!((*shared).vec.len(), len);
        assert_eq!((*shared).vec[0], 1);
    }

    unsafe {
        // Clean up to avoid memory leak
        let _ = Box::from_raw(byte_mut.data); 
    }
}

#[test]
#[should_panic(expected = "assertion failed")]
fn test_promote_to_shared_with_ref_cnt_2() {
    use std::sync::atomic::AtomicUsize;

    struct Shared {
        vec: Vec<u8>,
        original_capacity_repr: usize,
        ref_count: AtomicUsize,
    }

    struct BytesMut {
        data: *mut Shared,
        len: usize,
        cap: usize,
    }

    const KIND_VEC: usize = 0;
    
    let vec = vec![4, 5, 6];
    let len = vec.len();
    let cap = vec.capacity();
    let original_capacity_repr = 0;

    let mut byte_mut = BytesMut {
        data: std::ptr::null_mut(),
        len,
        cap,
    };

    byte_mut.data = Box::into_raw(Box::new(Shared {
        vec: vec,
        original_capacity_repr,
        ref_count: AtomicUsize::new(2), // ref_cnt set to 2 here, should trigger panic
    }));

    unsafe {
        byte_mut.promote_to_shared(2); // Testing with ref_cnt == 2, which should panic
    }

    unsafe {
        let _ = Box::from_raw(byte_mut.data); 
    }
}

