// Answer 0

#[test]
fn test_set_ctrl_with_valid_index() {
    struct RawTableInner {
        bucket_mask: usize,
        ctrl: Vec<Tag>,
    }

    impl RawTableInner {
        fn buckets(&self) -> usize {
            self.bucket_mask + 1
        }

        unsafe fn ctrl(&mut self, index: usize) -> &mut Tag {
            &mut self.ctrl[index]
        }
    }

    #[derive(Copy, Clone)]
    enum Tag {
        EMPTY,
        FILLED,
    }

    let mut table = RawTableInner {
        bucket_mask: 3, // Example where number of buckets is 4 (0-3)
        ctrl: vec![Tag::EMPTY; 8], // 8 control bytes for safety
    };

    unsafe {
        table.set_ctrl(2, Tag::FILLED); // Should not panic
        assert_eq!(table.ctrl[2], Tag::FILLED);
        assert_eq!(table.ctrl[6], Tag::FILLED); // Should replicate
    }
}

#[test]
#[should_panic] // Should panic if index is out of bounds
fn test_set_ctrl_with_index_too_high() {
    struct RawTableInner {
        bucket_mask: usize,
        ctrl: Vec<Tag>,
    }

    impl RawTableInner {
        fn buckets(&self) -> usize {
            self.bucket_mask + 1
        }

        unsafe fn ctrl(&mut self, index: usize) -> &mut Tag {
            &mut self.ctrl[index]
        }
    }

    #[derive(Copy, Clone)]
    enum Tag {
        EMPTY,
        FILLED,
    }

    let mut table = RawTableInner {
        bucket_mask: 3,
        ctrl: vec![Tag::EMPTY; 8],
    };

    unsafe {
        table.set_ctrl(4, Tag::FILLED); // Should panic, index 4 is out of bounds
    }
}

#[test]
fn test_set_ctrl_with_index_at_boundaries() {
    struct RawTableInner {
        bucket_mask: usize,
        ctrl: Vec<Tag>,
    }

    impl RawTableInner {
        fn buckets(&self) -> usize {
            self.bucket_mask + 1
        }

        unsafe fn ctrl(&mut self, index: usize) -> &mut Tag {
            &mut self.ctrl[index]
        }
    }

    #[derive(Copy, Clone)]
    enum Tag {
        EMPTY,
        FILLED,
    }

    let mut table = RawTableInner {
        bucket_mask: 3,
        ctrl: vec![Tag::EMPTY; 8],
    };

    unsafe {
        table.set_ctrl(0, Tag::FILLED); // Set first control byte
        assert_eq!(table.ctrl[0], Tag::FILLED);
        assert_eq!(table.ctrl[4], Tag::FILLED); // Should replicate to index 4
        
        table.set_ctrl(3, Tag::FILLED); // Set last control byte
        assert_eq!(table.ctrl[3], Tag::FILLED);
        assert_eq!(table.ctrl[7], Tag::FILLED); // Should replicate to index 7
    }
}

