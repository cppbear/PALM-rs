// Answer 0

#[test]
fn test_shared_to_vec_with_unique_shared() {
    use std::alloc::{alloc, dealloc, Layout};
    use std::ptr::null_mut;

    unsafe {
        let layout = Layout::from_size_align(10, 1).unwrap();
        let shared_buf = alloc(layout);
        let shared = Box::new(Shared {
            buf: shared_buf,
            cap: 10,
            ref_cnt: AtomicUsize::new(1),
        });

        let ptr: *const u8 = shared_buf as *const u8;
        let atomic_ptr = AtomicPtr::new(Box::into_raw(shared) as *mut ());

        let vec = shared_to_vec(&atomic_ptr, ptr, 10);
        assert_eq!(vec.len(), 10);

        for i in 0..10 {
            vec[i] = i as u8;
        }

        dealloc(shared_buf, layout);
    }
}

#[test]
fn test_shared_to_vec_with_non_unique_shared() {
    use std::alloc::{alloc, dealloc, Layout};
    use std::ptr::null_mut;

    unsafe {
        let layout = Layout::from_size_align(10, 1).unwrap();
        let shared_buf = alloc(layout);
        let shared = Box::new(Shared {
            buf: shared_buf,
            cap: 10,
            ref_cnt: AtomicUsize::new(2),
        });

        let ptr: *const u8 = shared_buf as *const u8;
        let atomic_ptr = AtomicPtr::new(Box::into_raw(shared) as *mut ());

        let vec = shared_to_vec(&atomic_ptr, ptr, 10);
        assert_eq!(vec.len(), 10);

        for i in 0..10 {
            assert_eq!(vec[i], 0); // When not unique, we expect default values
        }

        dealloc(shared_buf, layout);
    }
}

