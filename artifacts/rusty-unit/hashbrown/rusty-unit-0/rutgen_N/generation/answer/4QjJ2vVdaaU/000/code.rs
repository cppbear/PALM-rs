// Answer 0

#[derive(Debug)]
struct Group {
    stride: usize,
    pos: usize,
}

impl Group {
    const WIDTH: usize = 4; // Assuming a width for demonstration
}

#[test]
fn test_move_next_within_bounds() {
    let mut group = Group { stride: 0, pos: 0 };

    let bucket_mask = 20; // Example bucket mask

    group.move_next(bucket_mask);

    assert_eq!(group.stride, Group::WIDTH);
    assert_eq!(group.pos, (group.stride + group.pos) & bucket_mask);
}

#[test]
#[should_panic(expected = "Went past end of probe sequence")]
fn test_move_next_exceeds_bounds() {
    let mut group = Group { stride: 21, pos: 0 }; // Initializing stride beyond bucket_mask

    let bucket_mask = 20;

    group.move_next(bucket_mask);
}

