// Answer 0

#[test]
fn test_promotable_to_vec_valid_case() {
    use std::mem::MaybeUninit;
    use std::ptr::null_mut;

    struct TestStruct {
        buffer: *mut u8,
        cap: usize,
        ref_cnt: AtomicUsize,
    }

    let len: usize = 10;
    let mut buf: Vec<u8> = vec![0; len];
    let mut shared = Box::new(TestStruct {
        buffer: buf.as_mut_ptr(),
        cap: buf.len(),
        ref_cnt: AtomicUsize::new(1),
    });

    let data = AtomicPtr::new(Box::into_raw(shared) as *mut ());
    let ptr: *const u8 = buf.as_ptr();

    let result = unsafe {
        promotable_to_vec(
            &data,
            ptr,
            len,
            |shared| {
                let shared_struct = unsafe { &*(shared as *mut TestStruct) };
                shared_struct.buffer
            },
        )
    };

    // Ensure that the result has the same contents as the original buf
    assert_eq!(result, buf);
}

#[test]
fn test_promotable_to_vec_empty() {
    use std::mem::MaybeUninit;
    use std::ptr::null_mut;

    struct TestStruct {
        buffer: *mut u8,
        cap: usize,
        ref_cnt: AtomicUsize,
    }

    let len: usize = 0;
    let mut buf: Vec<u8> = vec![];

    let mut shared = Box::new(TestStruct {
        buffer: buf.as_mut_ptr(),
        cap: buf.len(),
        ref_cnt: AtomicUsize::new(1),
    });

    let data = AtomicPtr::new(Box::into_raw(shared) as *mut ());
    let ptr: *const u8 = buf.as_ptr();

    let result = unsafe {
        promotable_to_vec(
            &data,
            ptr,
            len,
            |shared| {
                let shared_struct = unsafe { &*(shared as *mut TestStruct) };
                shared_struct.buffer
            },
        )
    };

    assert_eq!(result, buf);
}

#[test]
fn test_promotable_to_vec_edge_case_large_length() {
    use std::mem::MaybeUninit;
    use std::ptr::null_mut;

    struct TestStruct {
        buffer: *mut u8,
        cap: usize,
        ref_cnt: AtomicUsize,
    }

    let len: usize = 1024; // Large length
    let mut buf: Vec<u8> = vec![1; len];

    let mut shared = Box::new(TestStruct {
        buffer: buf.as_mut_ptr(),
        cap: buf.len(),
        ref_cnt: AtomicUsize::new(1),
    });

    let data = AtomicPtr::new(Box::into_raw(shared) as *mut ());
    let ptr: *const u8 = buf.as_ptr();

    let result = unsafe {
        promotable_to_vec(
            &data,
            ptr,
            len,
            |shared| {
                let shared_struct = unsafe { &*(shared as *mut TestStruct) };
                shared_struct.buffer
            },
        )
    };

    assert_eq!(result, buf);
}

#[test]
#[should_panic]
fn test_promotable_to_vec_panic_on_invalid_data() {
    let len: usize = 1;
    let ptr: *const u8 = null_mut(); // Invalid pointer

    let data = AtomicPtr::new(null_mut());

    let _result = unsafe {
        promotable_to_vec(
            &data,
            ptr,
            len,
            |shared| null_mut(), // Invalid function pointer
        )
    };
}

#[test]
fn test_promotable_to_vec_multiple_consecutive_calls() {
    use std::mem::MaybeUninit;
    use std::ptr::null_mut;

    struct TestStruct {
        buffer: *mut u8,
        cap: usize,
        ref_cnt: AtomicUsize,
    }

    let len: usize = 5;
    let mut buf1: Vec<u8> = vec![1, 2, 3, 4, 5];
    let mut buf2: Vec<u8> = vec![6, 7, 8, 9, 10];

    let mut shared1 = Box::new(TestStruct {
        buffer: buf1.as_mut_ptr(),
        cap: buf1.len(),
        ref_cnt: AtomicUsize::new(1),
    });

    let mut shared2 = Box::new(TestStruct {
        buffer: buf2.as_mut_ptr(),
        cap: buf2.len(),
        ref_cnt: AtomicUsize::new(1),
    });

    let data1 = AtomicPtr::new(Box::into_raw(shared1) as *mut ());
    let data2 = AtomicPtr::new(Box::into_raw(shared2) as *mut ());

    let result1 = unsafe {
        promotable_to_vec(
            &data1,
            buf1.as_ptr(),
            len,
            |shared| {
                let shared_struct = unsafe { &*(shared as *mut TestStruct) };
                shared_struct.buffer
            },
        )
    };

    let result2 = unsafe {
        promotable_to_vec(
            &data2,
            buf2.as_ptr(),
            len,
            |shared| {
                let shared_struct = unsafe { &*(shared as *mut TestStruct) };
                shared_struct.buffer
            },
        )
    };

    assert_eq!(result1, buf1);
    assert_eq!(result2, buf2);
}

