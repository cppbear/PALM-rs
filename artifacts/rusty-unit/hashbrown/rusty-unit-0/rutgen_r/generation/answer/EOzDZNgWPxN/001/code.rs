// Answer 0

fn find_insert_slot_in_group_test() {
    struct Group {
        empty_or_deleted_bitmap: u32,
    }

    impl Group {
        fn match_empty_or_deleted(&self) -> &Self {
            self
        }

        fn lowest_set_bit(&self) -> Option<u32> {
            let lowest_bit = self.empty_or_deleted_bitmap.trailing_zeros();
            if lowest_bit < 32 {
                Some(1 << lowest_bit)
            } else {
                None
            }
        }
    }

    struct ProbeSeq {
        pos: usize,
    }

    struct HashTable {
        bucket_mask: usize,
    }

    impl HashTable {
        fn find_insert_slot_in_group(&self, group: &Group, probe_seq: &ProbeSeq) -> Option<usize> {
            let bit = group.match_empty_or_deleted().lowest_set_bit();

            if bit.is_some() {
                Some((probe_seq.pos + bit.unwrap() as usize) & self.bucket_mask)
            } else {
                None
            }
        }
    }

    // Test Case 1: Normal case with empty and deleted spots
    let group1 = Group { empty_or_deleted_bitmap: 0b0000_0000_0000_0000_0000_0000_0000_0111 }; // position 0, 1, 2 are available
    let probe_seq1 = ProbeSeq { pos: 3 };
    let hash_table1 = HashTable { bucket_mask: 15 }; // 16 buckets (0-15)

    let result1 = hash_table1.find_insert_slot_in_group(&group1, &probe_seq1);
    assert_eq!(result1, Some(3)); // (3 + 1) & 15 = 3

    // Test Case 2: Check highest bit set
    let group2 = Group { empty_or_deleted_bitmap: 0b0000_0000_0000_0000_0000_0000_0001_0000 }; // position 4 is available
    let probe_seq2 = ProbeSeq { pos: 7 };
    let hash_table2 = HashTable { bucket_mask: 15 }; // 16 buckets (0-15)

    let result2 = hash_table2.find_insert_slot_in_group(&group2, &probe_seq2);
    assert_eq!(result2, Some(11)); // (7 + 16) & 15 = 11

    // Test Case 3: No available slots
    let group3 = Group { empty_or_deleted_bitmap: 0b0000_0000_0000_0000_0000_0000_1111_1111 }; // no free spots
    let probe_seq3 = ProbeSeq { pos: 0 };
    let hash_table3 = HashTable { bucket_mask: 15 }; // 16 buckets (0-15)

    let result3 = hash_table3.find_insert_slot_in_group(&group3, &probe_seq3);
    assert_eq!(result3, None); // No available slot

    // Test Case 4: Multiple slots available
    let group4 = Group { empty_or_deleted_bitmap: 0b0000_0000_0000_0000_0000_1110_0000_0000 }; // position 1, 2 are available
    let probe_seq4 = ProbeSeq { pos: 4 };
    let hash_table4 = HashTable { bucket_mask: 15 }; // 16 buckets (0-15)

    let result4 = hash_table4.find_insert_slot_in_group(&group4, &probe_seq4);
    assert_eq!(result4, Some(5)); // (4 + 2) & 15 = 6
}

