// Answer 0

#[test]
fn test_rehash_in_place_all_non_deleted() {
    struct TestTable {
        buckets: usize,
        ctrl: Vec<u8>,
        items: usize,
        growth_left: usize,
    }

    impl TestTable {
        fn buckets(&self) -> usize {
            self.buckets
        }

        fn ctrl(&self, i: usize) -> &u8 {
            &self.ctrl[i]
        }

        fn bucket_ptr(&self, i: usize, _size_of: usize) -> *mut u8 {
            // For testing, we will return a dummy pointer
            std::ptr::null_mut()
        }

        fn prepare_rehash_in_place(&mut self) {
            // Preparation logic for rehashing
        }

        fn find_insert_slot(&self, _hash: u64) -> usize {
            // For simplicity in our test, we return a valid position
            0
        }

        fn set_ctrl(&mut self, _i: usize, _tag: u8) {
            // Setting control byte
        }

        fn replace_ctrl_hash(&mut self, _i: usize, _hash: u64) -> u8 {
            // Return tag indicating that the slot was previously empty
            0
        }

        fn set_ctrl_hash(&mut self, _i: usize, _hash: u64) {
            // Set hash in control byte
        }
    }

    unsafe fn mock_hasher(_: &mut TestTable, _index: usize) -> u64 {
        // A simple mock hasher that always returns the same value
        42
    }

    let mut table = TestTable {
        buckets: 10,
        ctrl: vec![1; 10], // All buckets initialized to 1 (not DELETED)
        items: 5,
        growth_left: 5, // Example values
    };

    unsafe {
        table.rehash_in_place(&mock_hasher, std::mem::size_of::<u8>(), None);
    }
}

#[test]
#[should_panic]
fn test_rehash_in_place_with_deleted_element() {
    struct TestTable {
        buckets: usize,
        ctrl: Vec<u8>,
        items: usize,
        growth_left: usize,
    }

    impl TestTable {
        fn buckets(&self) -> usize {
            self.buckets
        }

        fn ctrl(&self, i: usize) -> &u8 {
            &self.ctrl[i]
        }

        fn bucket_ptr(&self, i: usize, _size_of: usize) -> *mut u8 {
            std::ptr::null_mut()
        }

        fn prepare_rehash_in_place(&mut self) {}

        fn find_insert_slot(&self, _hash: u64) -> usize {
            0
        }

        fn set_ctrl(&mut self, _i: usize, _tag: u8) {}

        fn replace_ctrl_hash(&mut self, _i: usize, _hash: u64) -> u8 {
            0
        }

        fn set_ctrl_hash(&mut self, _i: usize, _hash: u64) {}
    }

    unsafe fn mock_hasher(_: &mut TestTable, index: usize) -> u64 {
        // Simple mock hasher for testing
        index as u64
    }

    let mut table = TestTable {
        buckets: 10,
        ctrl: vec![0; 10], // Initialize with DELETED (indicated by 0)
        items: 5,
        growth_left: 5,
    };

    unsafe {
        table.rehash_in_place(&mock_hasher, std::mem::size_of::<u8>(), None);
    }
}

