// Answer 0

fn test_clear_no_drop_non_empty() {
    struct DummyHashTable {
        items: usize,
        bucket_mask: usize,
        growth_left: usize,
    }

    impl DummyHashTable {
        fn is_empty_singleton(&self) -> bool {
            self.items <= 1 // Assume 0 or 1 means empty singleton
        }

        fn ctrl(&mut self, _: usize) -> &mut [u8] {
            // Simulate control bytes as a mutable byte slice
            &mut [0; 16] // Assume a fixed size for control bytes
        }

        fn num_ctrl_bytes(&self) -> usize {
            16 // Fixed size for this test
        }
    }

    fn bucket_mask_to_capacity(mask: usize) -> usize {
        // Simulate capacity from bucket mask, for testing purposes
        mask + 1
    }

    let mut table = DummyHashTable {
        items: 5, // Ensure the table is non-empty
        bucket_mask: 4, // Example bucket mask
        growth_left: 0,
    };

    table.clear_no_drop();

    assert_eq!(table.items, 0); // Check if items are reset
    assert_eq!(table.growth_left, bucket_mask_to_capacity(table.bucket_mask)); // Validate growth left
}

fn main() {
    test_clear_no_drop_non_empty();
}

