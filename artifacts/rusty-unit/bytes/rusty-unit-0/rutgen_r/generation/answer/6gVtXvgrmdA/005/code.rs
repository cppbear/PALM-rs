// Answer 0

#[test]
fn test_promote_to_shared_ref_count_1() {
    struct MyBytesMut {
        data: *mut Shared,
        len: usize,
        cap: usize,
    }
    
    struct Shared {
        vec: Vec<u8>,
        original_capacity_repr: usize,
        ref_count: std::sync::atomic::AtomicUsize,
    }
    
    const KIND_VEC: usize = 0;
    const KIND_MASK: usize = 1;
    const KIND_ARC: usize = 2;
    const ORIGINAL_CAPACITY_MASK: usize = !0;
    const ORIGINAL_CAPACITY_OFFSET: usize = 0;
    const VEC_POS_OFFSET: usize = 0;

    impl MyBytesMut {
        fn kind(&self) -> usize {
            KIND_VEC
        }
    }

    unsafe {
        let mut bytes_mut = MyBytesMut {
            data: std::ptr::null_mut(),
            len: 5,
            cap: 10,
        };

        // Initialize shared with ref_cnt == 1
        bytes_mut.promote_to_shared(1);

        // Verify the shared pointer and ref_count
        let shared = bytes_mut.data as *mut Shared;
        assert!((*shared).ref_count.load(std::sync::atomic::Ordering::SeqCst) == 1);
        assert!((*shared).vec.len() == bytes_mut.len);
        assert!((*shared).vec.capacity() == bytes_mut.cap);
        assert_eq!((shared as usize) & KIND_MASK, KIND_ARC);
    }
}

#[test]
fn test_promote_to_shared_ref_count_2() {
    struct MyBytesMut {
        data: *mut Shared,
        len: usize,
        cap: usize,
    }

    struct Shared {
        vec: Vec<u8>,
        original_capacity_repr: usize,
        ref_count: std::sync::atomic::AtomicUsize,
    }

    const KIND_VEC: usize = 0;
    const KIND_MASK: usize = 1;
    const KIND_ARC: usize = 2;
    const ORIGINAL_CAPACITY_MASK: usize = !0;
    const ORIGINAL_CAPACITY_OFFSET: usize = 0;
    const VEC_POS_OFFSET: usize = 0;

    impl MyBytesMut {
        fn kind(&self) -> usize {
            KIND_VEC
        }
    }

    unsafe {
        let mut bytes_mut = MyBytesMut {
            data: std::ptr::null_mut(),
            len: 5,
            cap: 10,
        };

        // Initialize shared with ref_cnt == 2
        bytes_mut.promote_to_shared(2);

        // Verify the shared pointer and ref_count
        let shared = bytes_mut.data as *mut Shared;
        assert!((*shared).ref_count.load(std::sync::atomic::Ordering::SeqCst) == 2);
        assert!((*shared).vec.len() == bytes_mut.len);
        assert!((*shared).vec.capacity() == bytes_mut.cap);
        assert_eq!((shared as usize) & KIND_MASK, KIND_ARC);
    }
}

