// Answer 0

#[test]
fn test_shared_v_to_vec_unique() {
    use std::ptr::{self, null_mut};
    use std::sync::{Arc, atomic::{AtomicPtr, Ordering}};
    use std::mem;

    struct Shared {
        vec: Vec<u8>,
    }

    unsafe fn release_shared(shared: &mut Shared) {
        // Dummy implementation for the cleanup
    }

    let initial_vec = vec![1, 2, 3, 4, 5];
    let len = initial_vec.len();
    let mut shared = Shared {
        vec: initial_vec.clone(),
    };

    let shared_ptr = Arc::new(AtomicPtr::new(&mut shared as *mut _));
    let data = shared_ptr.clone();

    // Ensuring the `shared` is unique
    let unique_ptr = data.load(Ordering::Relaxed);
    assert!(!unique_ptr.is_null()); // Ensure there is a valid pointer

    let result = unsafe {
        shared_v_to_vec(&data, initial_vec.as_ptr(), len)
    };

    assert_eq!(result, initial_vec);
    assert!(shared.vec.is_empty()); // Verify the original shared.vec has been replaced
}

#[test]
#[should_panic]
fn test_shared_v_to_vec_not_unique() {
    use std::ptr::{self, null_mut};
    use std::sync::{Arc, atomic::{AtomicPtr, Ordering}};
    use std::mem;

    struct Shared {
        vec: Vec<u8>,
    }

    let shared = Shared {
        vec: vec![1, 2, 3],
    };

    let shared_ptr = Arc::new(AtomicPtr::new(&shared as *const _ as *mut _)); // Not unique

    let len = 3;
    let initial_data = vec![4, 5, 6];

    unsafe {
        let _ = shared_v_to_vec(&shared_ptr, initial_data.as_ptr(), len);
    }
}

