// Answer 0

#[test]
fn test_set_ctrl_hash_within_bounds() {
    struct RawTableInner {
        bucket_mask: usize,
    }

    impl RawTableInner {
        fn buckets(&self) -> usize {
            self.bucket_mask + 1
        }

        unsafe fn set_ctrl(&mut self, index: usize, tag: Tag) {
            // Dummy implementation for test purposes
        }

        unsafe fn set_ctrl_hash(&mut self, index: usize, hash: u64) {
            self.set_ctrl(index, Tag::full(hash));
        }
    }

    struct Tag;

    impl Tag {
        fn full(hash: u64) -> Self {
            Tag
        }
    }

    let mut table = RawTableInner { bucket_mask: 5 };
    let index = 3;
    let hash = 42;

    unsafe {
        table.set_ctrl_hash(index, hash);
    }
}

#[test]
#[should_panic]
fn test_set_ctrl_hash_out_of_bounds() {
    struct RawTableInner {
        bucket_mask: usize,
    }

    impl RawTableInner {
        fn buckets(&self) -> usize {
            self.bucket_mask + 1
        }

        unsafe fn set_ctrl(&mut self, index: usize, _tag: Tag) {
            // Dummy implementation for test purposes
            assert!(index <= self.bucket_mask, "Index out of bounds");
        }

        unsafe fn set_ctrl_hash(&mut self, index: usize, hash: u64) {
            self.set_ctrl(index, Tag::full(hash));
        }
    }

    struct Tag;

    impl Tag {
        fn full(hash: u64) -> Self {
            Tag
        }
    }

    let mut table = RawTableInner { bucket_mask: 5 };
    let index = 6; // out of bounds
    let hash = 42;

    unsafe {
        table.set_ctrl_hash(index, hash);
    }
}

