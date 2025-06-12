// Answer 0

#[test]
fn test_full_buckets_indices_valid_input() {
    struct MockRawTableInner {
        items: usize,
        control_bytes: Vec<u8>,
    }

    impl MockRawTableInner {
        fn ctrl(&self, index: usize) -> *const u8 {
            self.control_bytes.as_ptr().add(index)
        }

        fn buckets(&self) -> usize {
            self.control_bytes.len() / std::mem::size_of::<u8>()
        }
    }

    struct Group;

    impl Group {
        const WIDTH: usize = 4;

        unsafe fn load_aligned(ptr: *const u8) -> Self {
            // Assume loading group from provided pointer.
            Group
        }

        fn match_full(&self) -> Vec<u8> {
            // Return a vector representing full matched buckets.
            vec![1, 1, 1, 1] // represents full buckets
        }
    }
    
    struct FullBucketsIndices {
        current_group: std::slice::Iter<'static, u8>,
        group_first_index: usize,
        ctrl: std::ptr::NonNull<u8>,
        items: usize,
    }

    unsafe fn full_buckets_indices(inner: &MockRawTableInner) -> FullBucketsIndices {
        let ctrl = std::ptr::NonNull::new_unchecked(inner.ctrl(0));
        
        FullBucketsIndices {
            current_group: Group::load_aligned(ctrl.as_ptr())
                .match_full()
                .into_iter(),
            group_first_index: 0,
            ctrl,
            items: inner.items,
        }
    }

    let inner = MockRawTableInner {
        items: 4,
        control_bytes: vec![1, 1, 1, 1, 0, 0, 0, 0], // initialized for full buckets
    };

    let indices = unsafe { full_buckets_indices(&inner) };
    
    assert_eq!(indices.items, 4);
    assert_eq!(indices.group_first_index, 0);
    assert_eq!(indices.current_group.clone().collect::<Vec<_>>(), vec![1, 1, 1, 1]);
}

