// Answer 0

#[derive(Default)]
struct Group {
    pub(crate) width: usize,
}

impl Group {
    const WIDTH: usize = 1; // Assuming a WIDTH constant for the purpose of this example.
}

struct TestStruct {
    stride: usize,
    pos: usize,
}

impl TestStruct {
    fn new(stride: usize, pos: usize) -> Self {
        Self { stride, pos }
    }

    fn move_next(&mut self, bucket_mask: usize) {
        debug_assert!(
            self.stride <= bucket_mask,
            "Went past end of probe sequence"
        );

        self.stride += Group::WIDTH;
        self.pos += self.stride;
        self.pos &= bucket_mask;
    }
}

#[test]
fn test_move_next_max_stride() {
    let bucket_mask = 4; // Choose a bucket mask that allows testing of the edge case.
    let mut test_struct = TestStruct::new(bucket_mask, 0); // Initialize with stride equal to bucket_mask.

    test_struct.move_next(bucket_mask);

    assert_eq!(test_struct.stride, bucket_mask + Group::WIDTH);
    assert_eq!(test_struct.pos, test_struct.stride & bucket_mask);
}

#[test]
#[should_panic(expected = "Went past end of probe sequence")]
fn test_move_next_panic() {
    let bucket_mask = 2;
    let mut test_struct = TestStruct::new(bucket_mask + 1, 0); // Set stride greater than bucket_mask to trigger panic.

    test_struct.move_next(bucket_mask);
}

