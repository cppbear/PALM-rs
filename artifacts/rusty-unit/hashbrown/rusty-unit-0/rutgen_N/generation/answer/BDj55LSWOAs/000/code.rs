// Answer 0

#[test]
fn test_probe_seq_initialization() {
    struct TestTable {
        bucket_mask: usize,
    }

    impl TestTable {
        fn buckets(&self) -> usize {
            self.bucket_mask + 1
        }

        fn probe_seq(&self, hash: u64) -> ProbeSeq {
            ProbeSeq {
                pos: h1(hash) & self.bucket_mask,
                stride: 0,
            }
        }
    }

    struct ProbeSeq {
        pos: usize,
        stride: usize,
    }

    fn h1(hash: u64) -> usize {
        (hash % (1u64 << 32)) as usize // Example hash function; adjust as needed
    }

    let table = TestTable { bucket_mask: 15 }; // buckets() would be 16
    let hash_value = 12345;
    let probe_sequence = table.probe_seq(hash_value);

    assert_eq!(probe_sequence.pos, h1(hash_value) & table.bucket_mask);
    assert_eq!(probe_sequence.stride, 0);
}

#[test]
fn test_probe_seq_boundary_condition() {
    struct TestTable {
        bucket_mask: usize,
    }

    impl TestTable {
        fn buckets(&self) -> usize {
            self.bucket_mask + 1
        }

        fn probe_seq(&self, hash: u64) -> ProbeSeq {
            ProbeSeq {
                pos: h1(hash) & self.bucket_mask,
                stride: 0,
            }
        }
    }

    struct ProbeSeq {
        pos: usize,
        stride: usize,
    }

    fn h1(hash: u64) -> usize {
        (hash % (1u64 << 32)) as usize // Example hash function; adjust as needed
    }

    let table = TestTable { bucket_mask: 7 }; // buckets() would be 8
    let hash_value = 7; // Edge case to see handling at boundary

    let probe_sequence = table.probe_seq(hash_value);
    assert_eq!(probe_sequence.pos, h1(hash_value) & table.bucket_mask);
    assert_eq!(probe_sequence.stride, 0);
}

