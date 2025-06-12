// Answer 0

#[test]
fn test_new_raw_iter_hash_inner() {
    struct RawTableInner {
        bucket_mask: u64,
        ctrl: fn(usize) -> u64,
        // other necessary fields...
    }

    struct Tag {
        // necessary fields...
    }

    struct Group {
        // necessary fields...
    }

    struct ProbeSeq {
        pos: usize,
        // other necessary fields...
    }

    struct RawIterHashInner {
        bucket_mask: u64,
        ctrl: fn(usize) -> u64,
        tag_hash: Tag,
        probe_seq: ProbeSeq,
        group: Group,
        bitmask: Box<dyn Iterator<Item = u64>>,
    }

    impl Tag {
        fn full(hash: u64) -> Self {
            // Implement tag creation here
            Tag { /* initialize fields */ }
        }
    }

    impl Group {
        fn load(ctrl_hash: u64) -> Self {
            // Implement group loading here
            Group { /* initialize fields */ }
        }

        fn match_tag(&self, tag_hash: Tag) -> Vec<u64> {
            // Implement tag matching logic here
            vec![1, 2, 3] // Sample return value
        }
    }

    impl RawTableInner {
        fn probe_seq(&self, hash: u64) -> ProbeSeq {
            // Sample implementation of probe sequence
            ProbeSeq { pos: 0 } // Sample return value
        }
    }

    unsafe {
        let table = RawTableInner {
            bucket_mask: 0xFFFFFFFF,
            ctrl: |pos| pos as u64, // Sample control function
            // initialize other fields...
        };

        let hash = 12345; // Sample hash value
        let result = new(&table, hash);

        assert_eq!(result.bucket_mask, table.bucket_mask);
        assert_eq!(result.ctrl(0), table.ctrl(0));
        assert_eq!(result.tag_hash, Tag::full(hash));
        assert_eq!(result.probe_seq.pos, table.probe_seq(hash).pos);
        // You can validate `result.group` and `result.bitmask` as needed
    }
}

