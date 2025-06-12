// Answer 0

#[test]
fn test_move_next_panic_stride_greater_than_bucket_mask_1() {
    let bucket_mask = 1;
    let mut probe_seq = ProbeSeq { pos: 0, stride: 2 };
    probe_seq.move_next(bucket_mask);
}

#[test]
fn test_move_next_panic_stride_greater_than_bucket_mask_2() {
    let bucket_mask = 10;
    let mut probe_seq = ProbeSeq { pos: 0, stride: 11 };
    probe_seq.move_next(bucket_mask);
}

#[test]
fn test_move_next_panic_stride_greater_than_bucket_mask_max() {
    let bucket_mask = usize::MAX;
    let mut probe_seq = ProbeSeq { pos: 0, stride: usize::MAX - 1 };
    probe_seq.move_next(bucket_mask);
}

#[test]
fn test_move_next_bucket_mask_large_stride() {
    let bucket_mask = 100;
    let mut probe_seq = ProbeSeq { pos: 0, stride: 101 };
    probe_seq.move_next(bucket_mask);
}

#[test]
fn test_move_next_bucket_mask_max_with_large_stride() {
    let bucket_mask = usize::MAX;
    let mut probe_seq = ProbeSeq { pos: 0, stride: usize::MAX };
    probe_seq.move_next(bucket_mask);
}

#[test]
fn test_move_next_bucket_mask_one_with_large_stride() {
    let bucket_mask = 1;
    let mut probe_seq = ProbeSeq { pos: 0, stride: usize::MAX };
    probe_seq.move_next(bucket_mask);
}

