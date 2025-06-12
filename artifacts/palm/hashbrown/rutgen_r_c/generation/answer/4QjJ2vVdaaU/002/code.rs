// Answer 0

#[test]
#[should_panic(expected = "Went past end of probe sequence")]
fn test_move_next_panic_stride_exceeds_bucket_mask() {
    struct TestProbeSeq {
        pos: usize,
        stride: usize,
    }
    
    impl ProbeSeq {
        fn new(pos: usize, stride: usize) -> Self {
            TestProbeSeq { pos, stride }
        }

        fn move_next(&mut self, bucket_mask: usize) {
            debug_assert!(self.stride <= bucket_mask, "Went past end of probe sequence");
            self.stride += Group::WIDTH;
            self.pos += self.stride;
            self.pos &= bucket_mask;
        }
    }

    let bucket_mask = 2; // set a low bucket_mask
    let mut probe_seq = TestProbeSeq::new(0, 3); // stride initialized greater than bucket_mask

    probe_seq.move_next(bucket_mask); // this should panic
}

#[test]
fn test_move_next_success() {
    struct TestProbeSeq {
        pos: usize,
        stride: usize,
    }

    impl ProbeSeq {
        fn new(pos: usize, stride: usize) -> Self {
            TestProbeSeq { pos, stride }
        }

        fn move_next(&mut self, bucket_mask: usize) {
            debug_assert!(self.stride <= bucket_mask, "Went past end of probe sequence");
            self.stride += Group::WIDTH;
            self.pos += self.stride;
            self.pos &= bucket_mask;
        }
    }

    let bucket_mask = 5; // set a sufficient bucket_mask
    let mut probe_seq = TestProbeSeq::new(0, 2); // stride initialized within bucket_mask

    probe_seq.move_next(bucket_mask); // this should not panic

    assert_eq!(probe_seq.pos, 2); // check expected position after move_next
    assert_eq!(probe_seq.stride, 4); // check expected stride after move_next
}

