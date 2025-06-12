// Answer 0

#[test]
fn test_raw_iter_valid() {
    struct RawTableInner {
        ctrl: Vec<u8>,
        items: usize,
        data_end: *const u8,
    }

    struct RawIter<T> {
        iter: RawIterRange,
        items: usize,
    }

    struct RawIterRange {
        ctrl: *const u8,
        data: *const T,
        buckets: usize,
    }

    impl RawTableInner {
        fn buckets(&self) -> usize {
            self.ctrl.len() / std::mem::size_of::<u8>() // assuming control bytes are u8
        }

        fn data_end(&self) -> *const u8 {
            self.data_end
        }
    }

    // Assuming we are going to use i32 as the type T for simplicity
    let ctrl = vec![0u8; 16]; // Initializing control bytes
    let items = 4; // Assume we have 4 items
    let data = vec![1i32, 2, 3, 4]; // Container for the actual data
    let data_end = data.as_ptr() as *const u8;

    let table = RawTableInner {
        ctrl,
        items,
        data_end,
    };

    unsafe {
        let iter: RawIter<i32> = table.iter();
        assert_eq!(iter.items, items);
    }
}

#[test]
#[should_panic]
fn test_raw_iter_panics_if_ctrl_not_initialized() {
    struct RawTableInner {
        ctrl: Vec<u8>,
        items: usize,
        data_end: *const u8,
    }

    struct RawIter<T> {
        iter: RawIterRange,
        items: usize,
    }

    struct RawIterRange {
        ctrl: *const u8,
        data: *const T,
        buckets: usize,
    }

    impl RawTableInner {
        fn buckets(&self) -> usize {
            self.ctrl.len() / std::mem::size_of::<u8>() // assuming control bytes are u8
        }

        fn data_end(&self) -> *const u8 {
            self.data_end
        }
    }

    let uninitialized_ctrl = vec![]; // Control bytes not properly initialized
    let items = 0; // No items
    let data_end = std::ptr::null();

    let table = RawTableInner {
        ctrl: uninitialized_ctrl,
        items,
        data_end,
    };

    unsafe {
        let _iter: RawIter<i32> = table.iter(); // This should panic due to uninitialized control bytes
    }
}

