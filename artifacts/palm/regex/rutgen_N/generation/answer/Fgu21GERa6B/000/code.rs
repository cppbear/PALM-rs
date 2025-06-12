// Answer 0

#[derive(Default)]
struct TestStruct(InnerStruct);

#[derive(Default)]
struct InnerStruct {
    slots: Vec<usize>,
}

impl InnerStruct {
    fn slots_len(&self) -> usize {
        self.slots.len()
    }
}

impl TestStruct {
    fn slots_len(&self) -> usize {
        self.0.slots_len()
    }
}

#[test]
fn test_slots_len_empty() {
    let inner = InnerStruct::default();
    let test_struct = TestStruct(inner);
    assert_eq!(test_struct.slots_len(), 0);
}

#[test]
fn test_slots_len_with_elements() {
    let mut inner = InnerStruct::default();
    inner.slots.push(1);
    inner.slots.push(2);
    let test_struct = TestStruct(inner);
    assert_eq!(test_struct.slots_len(), 2);
}

#[test]
fn test_slots_len_large() {
    let mut inner = InnerStruct::default();
    for _ in 0..1000 {
        inner.slots.push(1);
    }
    let test_struct = TestStruct(inner);
    assert_eq!(test_struct.slots_len(), 1000);
}

