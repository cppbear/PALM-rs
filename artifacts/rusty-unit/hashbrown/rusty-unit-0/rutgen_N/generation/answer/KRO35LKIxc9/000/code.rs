// Answer 0

#[test]
fn test_ctrl_index_within_bounds() {
    struct RawTableInner {
        ctrl: Vec<u8>,
        bucket_mask: usize,
    }

    impl RawTableInner {
        fn num_ctrl_bytes(&self) -> usize {
            self.ctrl.len()
        }

        unsafe fn ctrl(&self, index: usize) -> *mut u8 {
            debug_assert!(index < self.num_ctrl_bytes());
            self.ctrl.as_ptr().add(index).cast()
        }
    }

    let ctrl_bytes = vec![0u8; 10];
    let raw_table = RawTableInner {
        ctrl: ctrl_bytes,
        bucket_mask: 8,
    };

    let index = 5;
    let ptr = unsafe { raw_table.ctrl(index) };
    assert!(!ptr.is_null());
}

#[test]
#[should_panic]
fn test_ctrl_index_out_of_bounds() {
    struct RawTableInner {
        ctrl: Vec<u8>,
        bucket_mask: usize,
    }

    impl RawTableInner {
        fn num_ctrl_bytes(&self) -> usize {
            self.ctrl.len()
        }

        unsafe fn ctrl(&self, index: usize) -> *mut u8 {
            debug_assert!(index < self.num_ctrl_bytes());
            self.ctrl.as_ptr().add(index).cast()
        }
    }

    let ctrl_bytes = vec![0u8; 10];
    let raw_table = RawTableInner {
        ctrl: ctrl_bytes,
        bucket_mask: 8,
    };

    let out_of_bounds_index = 10;
    unsafe { raw_table.ctrl(out_of_bounds_index) };
}

