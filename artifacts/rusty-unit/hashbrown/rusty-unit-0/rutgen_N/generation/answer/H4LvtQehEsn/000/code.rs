// Answer 0

#[cfg(test)]
mod tests {
    use super::*;
    use std::ptr::NonNull;

    struct MockRawTableInner {
        data: Vec<i32>,
        control_bytes: Vec<u8>,
        items: usize,
    }

    unsafe impl RawTableInner for MockRawTableInner {
        fn ctrl(&self, _index: usize) -> *const u8 {
            self.control_bytes.as_ptr()
        }

        fn items(&self) -> usize {
            self.items
        }
        
        fn buckets(&self) -> usize {
            self.control_bytes.len()
        }
    }

    #[test]
    fn test_full_buckets_indices_empty() {
        let raw_table = MockRawTableInner {
            data: Vec::new(),
            control_bytes: vec![0; 1],
            items: 0,
        };
        
        let indices = unsafe { raw_table.full_buckets_indices() };
        
        assert!(indices.current_group.len() == 0);
        assert_eq!(indices.group_first_index, 0);
    }

    #[test]
    #[should_panic]
    fn test_full_buckets_indices_uninitialized_control_bytes() {
        let raw_table = MockRawTableInner {
            data: Vec::new(),
            control_bytes: vec![],
            items: 0,
        };
        
        // This should panic due to uninitialized control bytes
        let _ = unsafe { raw_table.full_buckets_indices() };
    }

    #[test]
    fn test_full_buckets_indices_with_data() {
        let raw_table = MockRawTableInner {
            data: vec![1, 2, 3],
            control_bytes: vec![1, 1, 1], // assuming each indicates a full bucket
            items: 3,
        };
        
        let indices = unsafe { raw_table.full_buckets_indices() };

        assert!(indices.current_group.len() > 0);
        assert_eq!(indices.group_first_index, 0);
    }
}

