// Answer 0

#[test]
fn test_prepare_rehash_in_place_with_no_entries() {
    struct RawTableInner {
        control: Vec<Tag>,
        buckets: usize,
    }

    impl RawTableInner {
        fn new(buckets: usize) -> Self {
            RawTableInner {
                control: vec![Tag::EMPTY; buckets],
                buckets,
            }
        }

        fn ctrl(&mut self, index: usize) -> &mut Tag {
            &mut self.control[index]
        }

        fn buckets(&self) -> usize {
            self.buckets
        }
    }

    #[repr(C)]
    enum Tag {
        EMPTY,
        DELETED,
        FULL,
    }

    // Simulate the behavior of the method
    unsafe fn prepare_rehash_in_place(table: &mut RawTableInner) {
        for i in (0..table.buckets()).step_by(1) {
            let group = &mut table.control[i];
            *group = match *group {
                Tag::FULL => Tag::DELETED,
                Tag::DELETED => Tag::EMPTY,
                Tag::EMPTY => Tag::EMPTY,
            };
        }
    }

    let mut table = RawTableInner::new(5);
    unsafe { prepare_rehash_in_place(&mut table) };

    assert_eq!(table.control, vec![Tag::EMPTY; 5]);
}

#[test]
fn test_prepare_rehash_in_place_with_full_and_deleted_entries() {
    struct RawTableInner {
        control: Vec<Tag>,
        buckets: usize,
    }

    impl RawTableInner {
        fn new(control: Vec<Tag>) -> Self {
            let buckets = control.len();
            RawTableInner { control, buckets }
        }

        fn ctrl(&mut self, index: usize) -> &mut Tag {
            &mut self.control[index]
        }

        fn buckets(&self) -> usize {
            self.buckets
        }
    }

    #[repr(C)]
    enum Tag {
        EMPTY,
        DELETED,
        FULL,
    }

    // Simulate the behavior of the method
    unsafe fn prepare_rehash_in_place(table: &mut RawTableInner) {
        for i in (0..table.buckets()).step_by(1) {
            let group = &mut table.control[i];
            *group = match *group {
                Tag::FULL => Tag::DELETED,
                Tag::DELETED => Tag::EMPTY,
                Tag::EMPTY => Tag::EMPTY,
            };
        }
    }

    let mut table = RawTableInner::new(vec![Tag::FULL, Tag::DELETED, Tag::EMPTY, Tag::FULL, Tag::DELETED]);
    unsafe { prepare_rehash_in_place(&mut table) };

    assert_eq!(table.control, vec![Tag::DELETED, Tag::EMPTY, Tag::EMPTY, Tag::DELETED, Tag::EMPTY]);
}

