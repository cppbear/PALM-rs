// Answer 0

#[test]
fn test_rehash_in_place_panic_conditions() {
    struct TestTable {
        buckets: usize,
        control: Vec<Tag>,
        items: usize,
        growth_left: usize,
    }

    impl TestTable {
        fn buckets(&self) -> usize {
            self.buckets
        }

        fn ctrl(&self, i: usize) -> &Tag {
            &self.control[i]
        }

        fn set_ctrl(&mut self, i: usize, tag: Tag) {
            self.control[i] = tag;
        }

        fn set_items(&mut self, items: usize) {
            self.items = items;
        }

        fn bucket_ptr(&self, _i: usize, _size_of: usize) -> *mut u8 {
            // Pointer simulation; use a dummy as we won't actually dereference in this test.
            0 as *mut u8
        }

        fn prepare_rehash_in_place(&self) {
            // Prepare for rehashing in place; no operation in this test.
        }
    }

    let mut test_table = TestTable {
        buckets: 5,
        control: vec![Tag::DELETED; 5],
        items: 5,
        growth_left: 0,
    };

    unsafe {
        let hasher = |_: &mut TestTable, _| -> u64 { 0 };
        test_table.rehash_in_place(&hasher, std::mem::size_of::<u8>(), None);
    }
}

#[test]
#[should_panic]
fn test_rehash_in_place_invalid_buckets() {
    struct TestTable {
        buckets: usize,
        control: Vec<Tag>,
        items: usize,
        growth_left: usize,
    }

    impl TestTable {
        fn buckets(&self) -> usize {
            self.buckets
        }

        fn ctrl(&self, i: usize) -> &Tag {
            &self.control[i]
        }

        fn set_ctrl(&mut self, i: usize, tag: Tag) {
            self.control[i] = tag;
        }

        fn bucket_ptr(&self, _i: usize, _size_of: usize) -> *mut u8 {
            0 as *mut u8
        }

        fn prepare_rehash_in_place(&self) {
            // Prepare for rehashing in place; no operation in this test.
        }
    }

    let mut test_table = TestTable {
        buckets: 5,
        control: vec![Tag::DELETED; 5],
        items: 5,
        growth_left: 0,
    };

    test_table.set_items(10); // Exceeding the total items to trigger panic.

    unsafe {
        let hasher = |_: &mut TestTable, _| -> u64 { 0 };
        test_table.rehash_in_place(&hasher, std::mem::size_of::<u8>(), None);
    }
}

