// Answer 0

#[test]
fn test_shared_v_to_vec_unique() {
    use std::ptr::{self, null_mut};
    use std::sync::atomic::{AtomicPtr, Ordering};
    use std::mem::{self, MaybeUninit};

    struct Shared {
        is_unique: bool,
        vec: Vec<u8>,
    }

    impl Shared {
        fn new(vec: Vec<u8>, is_unique: bool) -> Self {
            Shared { is_unique, vec }
        }

        fn is_unique(&self) -> bool {
            self.is_unique
        }
    }

    let data = AtomicPtr::new(Box::into_raw(Box::new(Shared::new(vec![0; 10], true))));
    let ptr: *const u8 = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10].as_ptr();
    let len = 10;

    let result = unsafe { shared_v_to_vec(&data, ptr, len) };

    assert_eq!(result, vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);

    // Cleanup
    let shared_ptr = data.load(Ordering::Relaxed);
    if !shared_ptr.is_null() {
        let _ = unsafe { Box::from_raw(shared_ptr) };
    }
}

#[test]
fn test_shared_v_to_vec_non_unique() {
    use std::ptr::{self, null_mut};
    use std::sync::atomic::{AtomicPtr, Ordering};
    use std::mem::{self, MaybeUninit};

    struct Shared {
        is_unique: bool,
        vec: Vec<u8>,
    }

    impl Shared {
        fn new(vec: Vec<u8>, is_unique: bool) -> Self {
            Shared { is_unique, vec }
        }

        fn is_unique(&self) -> bool {
            self.is_unique
        }
    }

    let shared_data = Box::new(Shared::new(vec![0; 10], false));
    let data = AtomicPtr::new(Box::into_raw(shared_data));
    let ptr: *const u8 = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10].as_ptr();
    let len = 10;

    let result = unsafe { shared_v_to_vec(&data, ptr, len) };

    assert_eq!(result, vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);

    // Cleanup
    let shared_ptr = data.load(Ordering::Relaxed);
    if !shared_ptr.is_null() {
        let _ = unsafe { Box::from_raw(shared_ptr) };
    }
}

