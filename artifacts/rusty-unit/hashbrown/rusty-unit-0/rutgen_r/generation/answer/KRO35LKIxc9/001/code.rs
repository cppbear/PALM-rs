// Answer 0

#[test]
fn test_ctrl_with_valid_index() {
    struct RawTableInner {
        ctrl: Vec<Tag>,
        bucket_mask: usize,
    }

    impl RawTableInner {
        fn new(size: usize, bucket_mask: usize) -> Self {
            Self {
                ctrl: vec![Tag; size],
                bucket_mask,
            }
        }

        fn num_ctrl_bytes(&self) -> usize {
            self.ctrl.len()
        }

        unsafe fn ctrl(&self, index: usize) -> *mut Tag {
            debug_assert!(index < self.num_ctrl_bytes());
            self.ctrl.as_ptr().add(index).cast()
        }
    }

    let bucket_mask = 3;
    let size = bucket_mask + 1 + 4; // Group::WIDTH is 4
    let table = RawTableInner::new(size, bucket_mask);

    let valid_index = bucket_mask + 1; // This should not panic
    let ptr = unsafe { table.ctrl(valid_index) };
    assert!(!ptr.is_null());
}

#[test]
#[should_panic(expected = "assertion failed")]
fn test_ctrl_with_invalid_index_too_large() {
    struct RawTableInner {
        ctrl: Vec<Tag>,
        bucket_mask: usize,
    }

    impl RawTableInner {
        fn new(size: usize, bucket_mask: usize) -> Self {
            Self {
                ctrl: vec![Tag; size],
                bucket_mask,
            }
        }

        fn num_ctrl_bytes(&self) -> usize {
            self.ctrl.len()
        }

        unsafe fn ctrl(&self, index: usize) -> *mut Tag {
            debug_assert!(index < self.num_ctrl_bytes());
            self.ctrl.as_ptr().add(index).cast()
        }
    }

    let bucket_mask = 3;
    let size = bucket_mask + 1 + 4; // Group::WIDTH is 4
    let table = RawTableInner::new(size, bucket_mask);

    let invalid_index = bucket_mask + 2; // This should panic
    unsafe { table.ctrl(invalid_index) };
}

#[test]
#[should_panic(expected = "assertion failed")]
fn test_ctrl_with_index_equal_to_num_ctrl_bytes() {
    struct RawTableInner {
        ctrl: Vec<Tag>,
        bucket_mask: usize,
    }

    impl RawTableInner {
        fn new(size: usize, bucket_mask: usize) -> Self {
            Self {
                ctrl: vec![Tag; size],
                bucket_mask,
            }
        }

        fn num_ctrl_bytes(&self) -> usize {
            self.ctrl.len()
        }

        unsafe fn ctrl(&self, index: usize) -> *mut Tag {
            debug_assert!(index < self.num_ctrl_bytes());
            self.ctrl.as_ptr().add(index).cast()
        }
    }

    let bucket_mask = 3;
    let size = bucket_mask + 1 + 4; // Group::WIDTH is 4
    let table = RawTableInner::new(size, bucket_mask);

    let index_equal_to_num_ctrl_bytes = table.num_ctrl_bytes(); // This should panic
    unsafe { table.ctrl(index_equal_to_num_ctrl_bytes) };
}

