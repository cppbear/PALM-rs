// Answer 0

#[cfg(test)]
mod tests {
    use std::sync::atomic::{AtomicPtr, Ordering};
    use std::ptr::null_mut;
    use std::sync::Arc;

    struct Shared {
        ref_cnt: AtomicUsize,
    }

    #[test]
    fn test_shared_is_unique_when_unique() {
        let shared = Box::into_raw(Box::new(Shared {
            ref_cnt: AtomicUsize::new(1),
        }));

        let data = AtomicPtr::new(shared);
        unsafe {
            assert_eq!(shared_is_unique(&data), true);
        }

        // Clean up
        data.store(null_mut(), Ordering::Release);
        let _ = Box::from_raw(shared); // Reclaims the memory
    }

    #[test]
    fn test_shared_is_unique_when_not_unique() {
        let shared = Box::into_raw(Box::new(Shared {
            ref_cnt: AtomicUsize::new(2),
        }));

        let data = AtomicPtr::new(shared);
        unsafe {
            assert_eq!(shared_is_unique(&data), false);
        }

        // Clean up
        data.store(null_mut(), Ordering::Release);
        let _ = Box::from_raw(shared); // Reclaims the memory
    }
}

