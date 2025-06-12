// Answer 0

#[derive(Clone, Copy)]
struct Tag(u8);

struct RawTableInner {
    bucket_mask: usize,
    control: Vec<Tag>,
}

impl RawTableInner {
    fn buckets(&self) -> usize {
        self.bucket_mask + 1
    }

    fn ctrl(&mut self, index: usize) -> &mut Tag {
        &mut self.control[index]
    }

    fn new(bucket_mask: usize) -> Self {
        RawTableInner {
            bucket_mask,
            control: vec![Tag(0); bucket_mask + 1],
        }
    }
}

impl RawTableInner {
    unsafe fn set_ctrl(&mut self, index: usize, ctrl: Tag) {
        let index2 = ((index.wrapping_sub(Group::WIDTH)) & self.bucket_mask) + Group::WIDTH;
        *self.ctrl(index) = ctrl;
        *self.ctrl(index2) = ctrl;
    }
}

struct Group;

impl Group {
    const WIDTH: usize = 4; // Example width for testing
}

#[test]
fn test_set_ctrl_within_bounds() {
    let mut table = RawTableInner::new(7); // bucket_mask of 7 (8 buckets)
    unsafe { table.set_ctrl(2, Tag(1)) };
    assert_eq!(table.ctrl(2).0, 1);
    assert_eq!(table.ctrl(6).0, 1); // Expected replicated index
}

#[test]
fn test_set_ctrl_at_edge() {
    let mut table = RawTableInner::new(7);
    unsafe { table.set_ctrl(7, Tag(2)) }; // Edge case index
    assert_eq!(table.ctrl(7).0, 2);
    assert_eq!(table.ctrl(3).0, 2); // Expected replicated index
}

#[should_panic]
fn test_set_ctrl_out_of_bounds() {
    let mut table = RawTableInner::new(7);
    unsafe { table.set_ctrl(8, Tag(3)) }; // Should panic on out of bounds
}

#[test]
fn test_set_ctrl_with_minimum_size() {
    let mut table = RawTableInner::new(0);
    unsafe { table.set_ctrl(0, Tag(4)) };
    assert_eq!(table.ctrl(0).0, 4);
    assert_eq!(table.ctrl(4).0, 4); // Expected replicated index
}

