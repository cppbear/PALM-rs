// Answer 0

#[test]
fn test_promotable_to_vec_arc() {
    use core::ptr;
    use alloc::vec::Vec;

    struct DummyArc;
    let data = AtomicPtr::new(Box::into_raw(Box::new(DummyArc)) as *mut ());

    // Create a mock ptr with sample data and length
    let sample_data = [1u8, 2, 3, 4, 5];
    let ptr = sample_data.as_ptr();
    let len = sample_data.len();

    // Function to simulate returning a mutable buffer
    let f: fn(*mut ()) -> *mut u8 = |shared| {
        let buf = Box::into_raw(Box::new(vec![0u8; len])) as *mut u8;
        buf
    };

    unsafe {
        let result = promotable_to_vec(&data, ptr, len, f);
        assert_eq!(result, sample_data);
    }

    // Clean up
    unsafe { Box::from_raw(data.load(Ordering::Relaxed) as *mut DummyArc) };
}

#[test]
fn test_promotable_to_vec_vec() {
    use core::ptr;
    use alloc::vec::Vec;

    struct DummyVec {
        data: Vec<u8>,
    }

    let data = AtomicPtr::new(Box::into_raw(Box::new(DummyVec { data: vec![0u8; 5] })) as *mut ());

    // Create a mock ptr with sample data and length
    let sample_data = [1u8, 2, 3, 4, 5];
    let ptr = sample_data.as_ptr();
    let len = sample_data.len();

    // Function to simulate returning a mutable buffer
    let f: fn(*mut ()) -> *mut u8 = |shared| {
        let buf = Box::into_raw(Box::new(vec![0u8; len])) as *mut u8;
        buf
    };

    unsafe {
        let result = promotable_to_vec(&data, ptr, len, f);
        assert_eq!(result, sample_data);
    }

    // Clean up
    unsafe { Box::from_raw(data.load(Ordering::Relaxed) as *mut DummyVec) };
}

