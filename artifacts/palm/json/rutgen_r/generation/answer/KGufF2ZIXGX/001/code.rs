// Answer 0

#[derive(Debug)]
struct TestStruct {
    index: usize,
}

impl TestStruct {
    fn new(index: usize) -> Self {
        Self { index }
    }

    fn discard(&mut self) {
        self.index += 1;
    }
}

#[test]
fn test_discard_increments_index() {
    let mut test_struct = TestStruct::new(0);
    test_struct.discard();
    assert_eq!(test_struct.index, 1);
}

#[test]
fn test_discard_multiple_calls() {
    let mut test_struct = TestStruct::new(5);
    test_struct.discard();
    test_struct.discard();
    assert_eq!(test_struct.index, 7);
}

#[test]
fn test_discard_with_large_index() {
    let mut test_struct = TestStruct::new(usize::MAX - 1);
    test_struct.discard();
    assert_eq!(test_struct.index, usize::MAX);
}

#[test]
fn test_discard_edge_case() {
    let mut test_struct = TestStruct::new(usize::MAX);
    test_struct.discard();
    assert_eq!(test_struct.index, usize::MAX + 1); // This will panic due to usize overflow, expect it to go into a panic state as it's out of bounds for usize
}

