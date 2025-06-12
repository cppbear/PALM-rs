// Answer 0

#[test]
fn test_rehash_in_place_with_no_deleted_elements() {
    struct RawTableInner {
        buckets_len: usize,
        ctrl: Vec<Tag>,
        items: usize,
        bucket_mask: usize,
        growth_left: usize,
    }

    impl RawTableInner {
        pub fn new(buckets_len: usize) -> Self {
            let ctrl = vec![Tag::EMPTY; buckets_len];
            RawTableInner {
                buckets_len,
                ctrl,
                items: 0,
                bucket_mask: buckets_len - 1,
                growth_left: buckets_len,
            }
        }

        pub fn buckets(&self) -> usize {
            self.buckets_len
        }

        pub fn ctrl(&self, index: usize) -> &Tag {
            &self.ctrl[index]
        }

        pub fn set_ctrl(&mut self, index: usize, tag: Tag) {
            self.ctrl[index] = tag;
        }

        pub fn find_insert_slot(&self, _hash: u64) -> InsertSlot {
            InsertSlot { index: 0 } // Simplified for this test
        }

        pub fn bucket_ptr(&self, index: usize, _size_of: usize) -> *mut u8 {
            std::ptr::null_mut() // Simplified for this test
        }
    }

    let mut table = RawTableInner::new(8);
    let hasher = |_: &mut RawTableInner, _: usize| 0; // Simplified hasher

    unsafe {
        table.rehash_in_place(&hasher, std::mem::size_of::<u8>(), None);
    }

    // Test logic here
}

#[test]
fn test_rehash_in_place_with_deleted_elements() {
    struct RawTableInner {
        buckets_len: usize,
        ctrl: Vec<Tag>,
        items: usize,
        bucket_mask: usize,
        growth_left: usize,
    }

    impl RawTableInner {
        pub fn new(buckets_len: usize) -> Self {
            let ctrl = vec![Tag::EMPTY; buckets_len];
            RawTableInner {
                buckets_len,
                ctrl,
                items: 1,
                bucket_mask: buckets_len - 1,
                growth_left: buckets_len,
            }
        }

        pub fn buckets(&self) -> usize {
            self.buckets_len
        }

        pub fn ctrl(&self, index: usize) -> &Tag {
            &self.ctrl[index]
        }

        pub fn set_ctrl(&mut self, index: usize, tag: Tag) {
            self.ctrl[index] = tag;
        }

        pub fn find_insert_slot(&self, _hash: u64) -> InsertSlot {
            InsertSlot { index: 0 } // Simplified for this test
        }

        pub fn bucket_ptr(&self, index: usize, _size_of: usize) -> *mut u8 {
            std::ptr::null_mut() // Simplified for this test
        }
    }

    let mut table = RawTableInner::new(8);
    table.set_ctrl(0, Tag::DELETED);

    let hasher = |_: &mut RawTableInner, _: usize| 0; // Simplified hasher

    unsafe {
        table.rehash_in_place(&hasher, std::mem::size_of::<u8>(), None);
    }

    // Test logic here
}

