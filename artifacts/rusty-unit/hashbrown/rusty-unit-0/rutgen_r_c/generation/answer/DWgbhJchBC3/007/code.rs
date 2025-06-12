// Answer 0

#[test]
fn test_find_or_find_insert_slot_inner_ok() {
    struct TestAllocator;
    struct TestTableLayout;
    struct TestGroup;

    impl TestGroup {
        fn load() -> Self {
            TestGroup
        }

        fn match_tag(&self, _: Tag) -> BitMask {
            // Simulate a successful match scenario
            BitMask(0b0000_0011) // Assume two bits are set
        }

        fn match_empty(&self) -> BitMask {
            // Simulate that there are no empty slots
            BitMask(0b0000_0000) // No empty
        }
    }

    impl RawTableInner {
        unsafe fn ctrl(&self, pos: usize) -> *mut Tag {
            // Simulate providing control information
            &mut Tag::EMPTY as *mut _ // Just for the sake of example
        }

        fn probe_seq(&self, hash: u64) -> ProbeSeq {
            // Simulate probing sequence
            ProbeSeq { pos: (hash % 4) as usize, stride: 0 } // Assume 4 buckets
        }

        unsafe fn find_insert_slot_in_group(&self, _: &TestGroup, _: &ProbeSeq) -> Option<usize> {
            // Simulate that an insert slot is found
            Some(0) // Example index found
        }

        unsafe fn fix_insert_slot(&self, index: usize) -> InsertSlot {
            InsertSlot { index }
        }
    }

    let table = RawTableInner {
        ctrl: NonNull::new(&mut Tag::EMPTY as *mut Tag as *mut u8).unwrap(),
        bucket_mask: 3,
        growth_left: 10,
        items: 2,
    };

    let hash = 42; // Example hash value

    let result = unsafe {
        table.find_or_find_insert_slot_inner(hash, &mut |index| {
            index == 1 // Simulate that we are looking for index 1 which exists
        })
    };

    assert_eq!(result, Ok(1)); // Expect the return of the found index
}

#[test]
fn test_find_or_find_insert_slot_inner_insert_slot() {
    struct TestAllocator;
    struct TestTableLayout;
    struct TestGroup;

    impl TestGroup {
        fn load() -> Self {
            TestGroup
        }

        fn match_tag(&self, _: Tag) -> BitMask {
            // Simulate a successful match scenario, but none for this test
            BitMask(0b0000_0000) // No actual tags matched
        }

        fn match_empty(&self) -> BitMask {
            // Simulate that there are no empty slots
            BitMask(0b0000_0000) // No empty
        }
    }

    impl RawTableInner {
        unsafe fn ctrl(&self, pos: usize) -> *mut Tag {
            // Simulate control bytes from a raw table inner
            &mut Tag::EMPTY as *mut _ // Just for the sake of example
        }

        fn probe_seq(&self, hash: u64) -> ProbeSeq {
            // Simulate probing sequence
            ProbeSeq { pos: (hash % 4) as usize, stride: 0 } // Assume 4 buckets
        }

        unsafe fn find_insert_slot_in_group(&self, _: &TestGroup, _: &ProbeSeq) -> Option<usize> {
            // Simulate that an insert slot is confirmed
            Some(2) // Example found slots
        }

        unsafe fn fix_insert_slot(&self, index: usize) -> InsertSlot {
            InsertSlot { index }
        }
    }

    let table = RawTableInner {
        ctrl: NonNull::new(&mut Tag::EMPTY as *mut Tag as *mut u8).unwrap(),
        bucket_mask: 3,
        growth_left: 10,
        items: 0,
    };

    let hash = 42; // Example hash value

    let result = unsafe {
        table.find_or_find_insert_slot_inner(hash, &mut |index| {
            false // Simulate that no indices actually match the given criteria
        })
    };

    assert_eq!(result, Err(InsertSlot { index: 2 })); // Expect the insert slot to be index 2
}

