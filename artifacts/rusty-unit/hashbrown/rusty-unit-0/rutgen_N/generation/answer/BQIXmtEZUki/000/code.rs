// Answer 0

#[test]
fn test_prepare_insert_slot_empty_bucket() {
    #[derive(Default)]
    struct RawTableInner {
        ctrl: Vec<Tag>,
        bucket_mask: usize,
    }
    
    #[derive(Clone, Copy)]
    enum Tag {
        EMPTY,
        DELETED,
        OCCUPIED,
    }
    
    impl RawTableInner {
        fn buckets(&self) -> usize {
            self.bucket_mask + 1
        }
        
        fn find_insert_slot(&self, hash: u64) -> usize {
            // Simulating the presence of at least one empty bucket
            for (i, tag) in self.ctrl.iter().enumerate() {
                if *tag == Tag::EMPTY {
                    return i;
                }
            }
            panic!("No empty buckets available.");
        }
        
        fn set_ctrl_hash(&mut self, index: usize, hash: u64) {
            self.ctrl[index] = Tag::OCCUPIED; // Marking as occupied for the test
        }
        
        fn ctrl(&self, index: usize) -> &Tag {
            &self.ctrl[index]
        }
    }
    
    let mut table = RawTableInner {
        ctrl: vec![Tag::EMPTY; 10],
        bucket_mask: 9,
    };
    
    let (index, old_ctrl) = unsafe { table.prepare_insert_slot(1234) };
    
    assert_eq!(index, 0);
    assert_eq!(old_ctrl, Tag::EMPTY);
    assert_eq!(table.ctrl[index], Tag::OCCUPIED);
}

#[test]
fn test_prepare_insert_slot_deleted_bucket() {
    #[derive(Default)]
    struct RawTableInner {
        ctrl: Vec<Tag>,
        bucket_mask: usize,
    }
    
    #[derive(Clone, Copy)]
    enum Tag {
        EMPTY,
        DELETED,
        OCCUPIED,
    }
    
    impl RawTableInner {
        fn buckets(&self) -> usize {
            self.bucket_mask + 1
        }
        
        fn find_insert_slot(&self, hash: u64) -> usize {
            // Here we simulate a deleted bucket present
            for (i, tag) in self.ctrl.iter().enumerate() {
                if *tag == Tag::DELETED {
                    return i;
                }
            }
            panic!("No empty or deleted buckets available.");
        }
        
        fn set_ctrl_hash(&mut self, index: usize, hash: u64) {
            self.ctrl[index] = Tag::OCCUPIED; // Marking as occupied for the test
        }
        
        fn ctrl(&self, index: usize) -> &Tag {
            &self.ctrl[index]
        }
    }
    
    let mut table = RawTableInner {
        ctrl: vec![Tag::DELETED, Tag::EMPTY, Tag::EMPTY, Tag::EMPTY, Tag::EMPTY, Tag::EMPTY, Tag::EMPTY, Tag::EMPTY, Tag::EMPTY, Tag::EMPTY],
        bucket_mask: 9,
    };
    
    let (index, old_ctrl) = unsafe { table.prepare_insert_slot(5678) };
    
    assert_eq!(index, 0);
    assert_eq!(old_ctrl, Tag::DELETED);
    assert_eq!(table.ctrl[index], Tag::OCCUPIED);
}

#[should_panic]
#[test]
fn test_prepare_insert_slot_no_empty_or_deleted_buckets() {
    #[derive(Default)]
    struct RawTableInner {
        ctrl: Vec<Tag>,
        bucket_mask: usize,
    }
    
    #[derive(Clone, Copy)]
    enum Tag {
        EMPTY,
        DELETED,
        OCCUPIED,
    }
    
    impl RawTableInner {
        fn buckets(&self) -> usize {
            self.bucket_mask + 1
        }
        
        fn find_insert_slot(&self, hash: u64) -> usize {
            // Simulating no empty buckets
            for (i, tag) in self.ctrl.iter().enumerate() {
                if *tag == Tag::EMPTY || *tag == Tag::DELETED {
                    return i;
                }
            }
            panic!("No empty or deleted buckets available.");
        }
        
        fn set_ctrl_hash(&mut self, index: usize, hash: u64) {
            self.ctrl[index] = Tag::OCCUPIED; // Marking as occupied for the test
        }
        
        fn ctrl(&self, index: usize) -> &Tag {
            &self.ctrl[index]
        }
    }
    
    let mut table = RawTableInner {
        ctrl: vec![Tag::OCCUPIED; 10], // All buckets are occupied
        bucket_mask: 9,
    };
    
    unsafe { table.prepare_insert_slot(91011) };
}

