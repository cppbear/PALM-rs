// Answer 0

#[derive(Debug)]
struct MyStruct(u8);

impl MyStruct {
    fn has_empty(&self) -> bool {
        self.0 & 0b00000_1_00 > 0
    }
}

#[test]
fn test_has_empty_true() {
    let my_struct = MyStruct(0b00000_1_00); // This should return true
    assert!(my_struct.has_empty());
}

#[test]
fn test_has_empty_false() {
    let my_struct = MyStruct(0b00000_0_00); // This should return false
    assert!(!my_struct.has_empty());
}

#[test]
fn test_has_empty_boundary_case() {
    let my_struct = MyStruct(0b00000_1_01); // This should also return true
    assert!(my_struct.has_empty());
}

