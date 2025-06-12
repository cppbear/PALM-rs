// Answer 0

#[test]
fn test_allocation_info_non_empty_table() {
    struct RawTableInner {
        buckets: usize,
        ctrl: *mut u8,
    }

    impl RawTableInner {
        fn is_empty_singleton(&self) -> bool {
            // Simulate that the table is non-empty.
            false
        }

        fn buckets(&self) -> usize {
            self.buckets
        }
    }

    struct TableLayout;

    impl TableLayout {
        fn calculate_layout_for(&self, buckets: usize) -> Option<(Layout, usize)> {
            // Simulate layout calculation
            if buckets > 0 {
                Some((Layout::from_size_align(16 * buckets, 8).unwrap(), 16))
            } else {
                None
            }
        }
    }

    use std::alloc::Layout;
    use std::ptr::NonNull;

    let table_layout = TableLayout;
    let ctrl_memory = Box::into_raw(Box::new(0u8)); // Simulated allocated control memory
    let raw_table = RawTableInner {
        buckets: 5,
        ctrl: ctrl_memory,
    };

    unsafe {
        let (ptr, layout) = raw_table.allocation_info(table_layout);
        assert!(!ptr.as_ptr().is_null());
        assert_eq!(layout.size(), 16 * 5);
        assert_eq!(layout.align(), 8);
    }

    unsafe { Box::from_raw(ctrl_memory); } // Avoid memory leak
}

#[test]
#[should_panic]
fn test_allocation_info_empty_table() {
    struct RawTableInner {
        buckets: usize,
        ctrl: *mut u8,
    }

    impl RawTableInner {
        fn is_empty_singleton(&self) -> bool {
            // Simulate that the table is empty.
            true
        }

        fn buckets(&self) -> usize {
            self.buckets
        }
    }

    struct TableLayout;

    let table_layout = TableLayout;
    let raw_table = RawTableInner {
        buckets: 0,
        ctrl: std::ptr::null_mut(),
    };

    unsafe {
        let _ = raw_table.allocation_info(table_layout);
    }
}

