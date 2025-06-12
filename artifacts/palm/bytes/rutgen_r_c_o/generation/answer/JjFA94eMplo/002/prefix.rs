// Answer 0

#[test]
fn test_shared_to_mut_impl_single_ref_count() {
    let cap = 10;
    let shared_buf = Box::new(Shared {
        buf: Box::into_raw(Box::new([1u8; 10])),
        cap,
        ref_cnt: AtomicUsize::new(1),
    });
    let ptr = unsafe { (*shared_buf).buf };
    let len = 5;

    let result = unsafe { shared_to_mut_impl(Box::into_raw(shared_buf), ptr, len) };
}

#[test]
fn test_shared_to_mut_impl_single_ref_count_full_buffer() {
    let cap = 10;
    let shared_buf = Box::new(Shared {
        buf: Box::into_raw(Box::new([2u8; 10])),
        cap,
        ref_cnt: AtomicUsize::new(1),
    });
    let ptr = unsafe { (*shared_buf).buf };
    let len = 10;

    let result = unsafe { shared_to_mut_impl(Box::into_raw(shared_buf), ptr, len) };
}

#[test]
fn test_shared_to_mut_impl_single_ref_count_minimum_length() {
    let cap = 10;
    let shared_buf = Box::new(Shared {
        buf: Box::into_raw(Box::new([3u8; 10])),
        cap,
        ref_cnt: AtomicUsize::new(1),
    });
    let ptr = unsafe { (*shared_buf).buf };
    let len = 1;

    let result = unsafe { shared_to_mut_impl(Box::into_raw(shared_buf), ptr, len) };
} 

#[test]
fn test_shared_to_mut_impl_single_ref_count_edge_case() {
    let cap = 10;
    let shared_buf = Box::new(Shared {
        buf: Box::into_raw(Box::new([4u8; 10])),
        cap,
        ref_cnt: AtomicUsize::new(1),
    });
    let ptr = unsafe { (*shared_buf).buf };
    let len = 7;

    let result = unsafe { shared_to_mut_impl(Box::into_raw(shared_buf), ptr, len) };
} 

#[test]
#[should_panic]
fn test_shared_to_mut_impl_invalid_ref_count() {
    let cap = 10;
    let shared_buf = Box::new(Shared {
        buf: Box::into_raw(Box::new([5u8; 10])),
        cap,
        ref_cnt: AtomicUsize::new(2),
    });
    let ptr = unsafe { (*shared_buf).buf };
    let len = 5;

    let _result = unsafe { shared_to_mut_impl(Box::into_raw(shared_buf), ptr, len) };
} 

