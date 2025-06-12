// Answer 0

#[test]
fn test_move_next_with_valid_bucket_mask() {
    struct TestProbeSeq {
        pos: usize,
        stride: usize,
    }

    impl ProbeSeq for TestProbeSeq {
        fn move_next(&mut self, bucket_mask: usize) {
            debug_assert!(self.stride <= bucket_mask, "Went past end of probe sequence");
            self.stride += Group::WIDTH;
            self.pos += self.stride;
            self.pos &= bucket_mask;
        }
    }

    let mut probe_seq = TestProbeSeq { pos: 0, stride: 0 };
    let bucket_mask = 8; // Example mask that is valid
    
    probe_seq.move_next(bucket_mask);
    
    assert_eq!(probe_seq.pos, Group::WIDTH);
    assert_eq!(probe_seq.stride, Group::WIDTH);
}

#[test]
#[should_panic(expected = "Went past end of probe sequence")]
fn test_move_next_with_exceeding_stride() {
    struct TestProbeSeq {
        pos: usize,
        stride: usize,
    }

    impl ProbeSeq for TestProbeSeq {
        fn move_next(&mut self, bucket_mask: usize) {
            debug_assert!(self.stride <= bucket_mask, "Went past end of probe sequence");
            self.stride += Group::WIDTH;
            self.pos += self.stride;
            self.pos &= bucket_mask;
        }
    }

    let mut probe_seq = TestProbeSeq { pos: 0, stride: 9 }; // Setting stride to exceed bucket_mask
    let bucket_mask = 8; // Example mask that is valid
    
    probe_seq.move_next(bucket_mask);
}

