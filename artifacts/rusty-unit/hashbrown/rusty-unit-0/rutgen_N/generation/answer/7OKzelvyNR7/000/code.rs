// Answer 0

#[test]
fn test_find_insert_slot_empty_bucket() {
    struct RawTableInner {
        buckets: Vec<Option<u64>>,
        bucket_mask: usize,
        ctrl: Vec<u8>,
    }

    struct InsertSlot(usize);
    
    impl RawTableInner {
        fn new(size: usize) -> Self {
            RawTableInner {
                buckets: vec![None; size],
                bucket_mask: size - 1,
                ctrl: vec![0; size + 1], // initializing control bytes
            }
        }
        
        fn buckets(&self) -> usize {
            self.buckets.len()
        }
        
        unsafe fn ctrl(&self, pos: usize) -> &u8 {
            &self.ctrl[pos]
        }

        unsafe fn fix_insert_slot(&self, index: usize) -> InsertSlot {
            InsertSlot(index)
        }

        // This mock function is used for testing and will always yield an empty group.
        unsafe fn probe_seq(&self, _hash: u64) -> ProbeSeq {
            ProbeSeq { pos: 0 }
        }

        unsafe fn find_insert_slot_in_group(&self, _group: &Group, probe_seq: &ProbeSeq) -> Option<usize> {
            if self.buckets[probe_seq.pos].is_none() {
                Some(probe_seq.pos)
            } else {
                None
            }
        }
    }

    struct ProbeSeq {
        pos: usize,
    }

    struct Group;

    impl Group {
        unsafe fn load(_ctrl: &u8) -> Self {
            Group
        }
    }

    let table = RawTableInner::new(4); // power of two size

    unsafe {
        let result = table.find_insert_slot(0);
        assert_eq!(result.0, 0);
    }
}

#[test]
fn test_find_insert_slot_full_table() {
    struct RawTableInner {
        buckets: Vec<Option<u64>>,
        bucket_mask: usize,
        ctrl: Vec<u8>,
    }

    struct InsertSlot(usize);
    
    impl RawTableInner {
        fn new(size: usize) -> Self {
            RawTableInner {
                buckets: vec![Some(1), Some(2), Some(3), Some(4)],
                bucket_mask: size - 1,
                ctrl: vec![1; size + 1], // initializing control bytes 
            }
        }
        
        fn buckets(&self) -> usize {
            self.buckets.len()
        }
        
        unsafe fn ctrl(&self, pos: usize) -> &u8 {
            &self.ctrl[pos]
        }

        unsafe fn fix_insert_slot(&self, index: usize) -> InsertSlot {
            InsertSlot(index)
        }

        unsafe fn probe_seq(&self, _hash: u64) -> ProbeSeq {
            ProbeSeq { pos: 0 }
        }

        unsafe fn find_insert_slot_in_group(&self, _group: &Group, probe_seq: &ProbeSeq) -> Option<usize> {
            if self.buckets[probe_seq.pos].is_none() {
                Some(probe_seq.pos)
            } else {
                None
            }
        }
    }

    struct ProbeSeq {
        pos: usize,
    }

    struct Group;

    impl Group {
        unsafe fn load(_ctrl: &u8) -> Self {
            Group
        }
    }

    let table = RawTableInner::new(4); // power of two size

    // This case will panic since there are no empty slots
    unsafe {
        let result = std::panic::catch_unwind(|| {
            table.find_insert_slot(0);
        });
        assert!(result.is_err());
    }
}

