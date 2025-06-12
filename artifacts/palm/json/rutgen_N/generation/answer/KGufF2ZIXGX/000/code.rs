// Answer 0

#[derive(Debug)]
struct TestStruct {
    index: usize,
}

impl TestStruct {
    fn new(index: usize) -> Self {
        TestStruct { index }
    }

    fn discard(&mut self) {
        self.index += 1;
    }
}

#[test]
fn test_discard_increments_index() {
    let mut obj = TestStruct::new(0);
    obj.discard();
    assert_eq!(obj.index, 1);
}

#[test]
fn test_discard_multiple_calls() {
    let mut obj = TestStruct::new(2);
    obj.discard();
    obj.discard();
    assert_eq!(obj.index, 4);
}

#[test]
fn test_discard_boundary_condition() {
    let mut obj = TestStruct::new(usize::MAX);
    obj.discard();
    assert_eq!(obj.index, usize::MAX + 1);
}

