// Answer 0

#[test]
fn test_prepare_insert_slot_success_empty_slot() {
    struct RawTableInner {
        buckets: Vec<Tag>,
        bucket_mask: usize,
    }

    impl RawTableInner {
        fn buckets(&self) -> usize {
            self.buckets.len()
        }

        fn find_insert_slot(&self, _hash: u64) -> usize {
            // Simulate finding an empty slot at index 0
            0
        }

        fn ctrl(&self, index: usize) -> &Tag {
            &self.buckets[index]
        }

        fn set_ctrl_hash(&mut self, index: usize, hash: u64) {
            // Simulate setting control hash
            self.buckets[index] = Tag::from_hash(hash);
        }
    }

    #[derive(Copy, Clone)]
    enum Tag {
        EMPTY,
        DELETED,
        OCCUPIED(u64),
    }

    impl Tag {
        fn from_hash(hash: u64) -> Tag {
            Tag::OCCUPIED(hash)
        }
    }

    let mut table = RawTableInner {
        buckets: vec![Tag::EMPTY, Tag::DELETED, Tag::EMPTY],
        bucket_mask: 2,
    };

    let (index, old_ctrl) = unsafe { table.prepare_insert_slot(42) };
    assert_eq!(index, 0);
    assert_eq!(old_ctrl, Tag::EMPTY);
}

#[test]
fn test_prepare_insert_slot_success_deleted_slot() {
    struct RawTableInner {
        buckets: Vec<Tag>,
        bucket_mask: usize,
    }

    impl RawTableInner {
        fn buckets(&self) -> usize {
            self.buckets.len()
        }

        fn find_insert_slot(&self, _hash: u64) -> usize {
            // Simulate finding a deleted slot at index 1
            1
        }

        fn ctrl(&self, index: usize) -> &Tag {
            &self.buckets[index]
        }

        fn set_ctrl_hash(&mut self, index: usize, hash: u64) {
            // Simulate setting control hash
            self.buckets[index] = Tag::from_hash(hash);
        }
    }

    #[derive(Copy, Clone)]
    enum Tag {
        EMPTY,
        DELETED,
        OCCUPIED(u64),
    }

    impl Tag {
        fn from_hash(hash: u64) -> Tag {
            Tag::OCCUPIED(hash)
        }
    }

    let mut table = RawTableInner {
        buckets: vec![Tag::EMPTY, Tag::DELETED, Tag::EMPTY],
        bucket_mask: 2,
    };

    let (index, old_ctrl) = unsafe { table.prepare_insert_slot(42) };
    assert_eq!(index, 1);
    assert_eq!(old_ctrl, Tag::DELETED);
}

#[should_panic]
#[test]
fn test_prepare_insert_slot_no_empty_or_deleted_slot() {
    struct RawTableInner {
        buckets: Vec<Tag>,
        bucket_mask: usize,
    }

    impl RawTableInner {
        fn buckets(&self) -> usize {
            self.buckets.len()
        }

        fn find_insert_slot(&self, _hash: u64) -> usize {
            // Simulate finding no empty slots
            panic!();
        }

        fn ctrl(&self, _index: usize) -> &Tag {
            &Tag::OCCUPIED(0) // Simulate no empty or deleted tag
        }

        fn set_ctrl_hash(&mut self, _index: usize, _hash: u64) {
            // No-op
        }
    }

    #[derive(Copy, Clone)]
    enum Tag {
        EMPTY,
        DELETED,
        OCCUPIED(u64),
    }

    let mut table = RawTableInner {
        buckets: vec![Tag::OCCUPIED(1), Tag::OCCUPIED(2)],
        bucket_mask: 1,
    };

    // This will panic since there are no available slots
    unsafe { table.prepare_insert_slot(42) };
}

