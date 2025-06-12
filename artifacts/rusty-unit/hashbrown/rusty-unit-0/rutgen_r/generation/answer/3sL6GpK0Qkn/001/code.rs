// Answer 0

#[test]
fn test_set_ctrl_hash_valid_index() {
    struct RawTableInner {
        bucket_mask: usize,
    }

    impl RawTableInner {
        fn buckets(&self) -> usize {
            self.bucket_mask + 1
        }

        unsafe fn set_ctrl(&mut self, _index: usize, _value: Tag) {
            // Simulate setting control value; implementation is not relevant for the test
        }
        
        unsafe fn set_ctrl_hash(&mut self, index: usize, hash: u64) {
            self.set_ctrl(index, Tag::full(hash));
        }
    }

    struct Tag(u64);

    impl Tag {
        fn full(hash: u64) -> Self {
            Tag(hash)
        }
    }

    let mut table = RawTableInner { bucket_mask: 5 };
    let index = 5; // Maximum valid index
    let hash = 12345;

    unsafe {
        table.set_ctrl_hash(index, hash);
    }
}

#[test]
#[should_panic]
fn test_set_ctrl_hash_index_too_large() {
    struct RawTableInner {
        bucket_mask: usize,
    }

    impl RawTableInner {
        fn buckets(&self) -> usize {
            self.bucket_mask + 1
        }

        unsafe fn set_ctrl(&mut self, _index: usize, _value: Tag) {
            // Implementation not relevant for the test
        }

        unsafe fn set_ctrl_hash(&mut self, index: usize, hash: u64) {
            self.set_ctrl(index, Tag::full(hash));
        }
    }

    struct Tag(u64);

    impl Tag {
        fn full(hash: u64) -> Self {
            Tag(hash)
        }
    }

    let mut table = RawTableInner { bucket_mask: 5 };
    let index = 6; // Invalid index (too large)
    let hash = 12345;

    unsafe {
        table.set_ctrl_hash(index, hash); // This should panic
    }
}

#[test]
fn test_set_ctrl_hash_edge_case() {
    struct RawTableInner {
        bucket_mask: usize,
    }

    impl RawTableInner {
        fn buckets(&self) -> usize {
            self.bucket_mask + 1
        }

        unsafe fn set_ctrl(&mut self, _index: usize, _value: Tag) {
            // Implementation not relevant for the test
        }

        unsafe fn set_ctrl_hash(&mut self, index: usize, hash: u64) {
            self.set_ctrl(index, Tag::full(hash));
        }
    }

    struct Tag(u64);

    impl Tag {
        fn full(hash: u64) -> Self {
            Tag(hash)
        }
    }

    let mut table = RawTableInner { bucket_mask: 0 };
    let index = 0; // Minimum valid index
    let hash = 67890;

    unsafe {
        table.set_ctrl_hash(index, hash);
    }
}

