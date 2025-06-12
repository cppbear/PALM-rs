// Answer 0

#[test]
fn test_shared_to_vec_impl_unique_ref_count() {
    let mut shared_buf: Vec<u8> = vec![0; 10];
    let shared_cap = shared_buf.len();
    
    let shared = Box::into_raw(Box::new(Shared {
        buf: shared_buf.as_mut_ptr(),
        cap: shared_cap,
        ref_cnt: AtomicUsize::new(1),
    }));
    
    let ptr: *const u8 = shared_buf.as_ptr();
    let len: usize = 5;

    unsafe {
        let result_vec = shared_to_vec_impl(shared, ptr, len);
    }
}

#[test]
fn test_shared_to_vec_impl_non_unique_ref_count() {
    let mut shared_buf: Vec<u8> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let shared_cap = shared_buf.len();
    
    let shared = Box::into_raw(Box::new(Shared {
        buf: shared_buf.as_mut_ptr(),
        cap: shared_cap,
        ref_cnt: AtomicUsize::new(2),
    }));

    let ptr: *const u8 = shared_buf.as_ptr();
    let len: usize = 10;

    unsafe {
        let result_vec = shared_to_vec_impl(shared, ptr, len);
    }
}

#[test]
fn test_shared_to_vec_impl_zero_length() {
    let mut shared_buf: Vec<u8> = vec![0; 10];
    let shared_cap = shared_buf.len();
    
    let shared = Box::into_raw(Box::new(Shared {
        buf: shared_buf.as_mut_ptr(),
        cap: shared_cap,
        ref_cnt: AtomicUsize::new(1),
    }));

    let ptr: *const u8 = shared_buf.as_ptr();
    let len: usize = 0;

    unsafe {
        let result_vec = shared_to_vec_impl(shared, ptr, len);
    }
}

#[test]
fn test_shared_to_vec_impl_full_capacity() {
    let mut shared_buf: Vec<u8> = vec![0; 15];
    let shared_cap = shared_buf.len();
    
    let shared = Box::into_raw(Box::new(Shared {
        buf: shared_buf.as_mut_ptr(),
        cap: shared_cap,
        ref_cnt: AtomicUsize::new(1),
    }));

    let ptr: *const u8 = shared_buf.as_ptr();
    let len: usize = shared_cap;

    unsafe {
        let result_vec = shared_to_vec_impl(shared, ptr, len);
    }
}

