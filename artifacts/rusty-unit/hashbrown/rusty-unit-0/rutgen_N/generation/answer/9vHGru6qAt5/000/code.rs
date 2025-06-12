// Answer 0

#[test]
fn test_clear_no_drop_non_empty() {
    struct TestTable {
        items: usize,
        growth_left: usize,
        bucket_mask: usize,
    }

    impl TestTable {
        fn is_empty_singleton(&self) -> bool {
            self.items == 0 && self.growth_left == 0
        }

        unsafe fn ctrl(&self, _index: usize) -> *mut u8 {
            // Returning a dummy pointer for illustration.
            std::ptr::null_mut()
        }

        fn num_ctrl_bytes(&self) -> usize {
            // Returning a fixed size for illustration.
            4
        }

        fn clear_no_drop(&mut self) {
            if !self.is_empty_singleton() {
                unsafe {
                    self.ctrl(0)
                        .write_bytes(0, self.num_ctrl_bytes());
                }
            }
            self.items = 0;
            self.growth_left = bucket_mask_to_capacity(self.bucket_mask);
        }
    }

    fn bucket_mask_to_capacity(mask: usize) -> usize {
        mask.count_ones() as usize
    }

    let mut table = TestTable {
        items: 5,
        growth_left: 3,
        bucket_mask: 0b111,
    };

    table.clear_no_drop();

    assert_eq!(table.items, 0);
    assert_eq!(table.growth_left, bucket_mask_to_capacity(table.bucket_mask));
}

#[test]
fn test_clear_no_drop_empty() {
    struct TestTable {
        items: usize,
        growth_left: usize,
        bucket_mask: usize,
    }

    impl TestTable {
        fn is_empty_singleton(&self) -> bool {
            self.items == 0 && self.growth_left == 0
        }

        unsafe fn ctrl(&self, _index: usize) -> *mut u8 {
            std::ptr::null_mut()
        }

        fn num_ctrl_bytes(&self) -> usize {
            4
        }

        fn clear_no_drop(&mut self) {
            if !self.is_empty_singleton() {
                unsafe {
                    self.ctrl(0)
                        .write_bytes(0, self.num_ctrl_bytes());
                }
            }
            self.items = 0;
            self.growth_left = bucket_mask_to_capacity(self.bucket_mask);
        }
    }

    fn bucket_mask_to_capacity(mask: usize) -> usize {
        mask.count_ones() as usize
    }

    let mut table = TestTable {
        items: 0,
        growth_left: 0,
        bucket_mask: 0b111,
    };

    table.clear_no_drop();

    assert_eq!(table.items, 0);
    assert_eq!(table.growth_left, bucket_mask_to_capacity(table.bucket_mask));
}

