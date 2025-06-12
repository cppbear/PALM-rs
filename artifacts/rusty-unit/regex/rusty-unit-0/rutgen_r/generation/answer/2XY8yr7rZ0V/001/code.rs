// Answer 0

#[derive(Default)]
struct MyStruct(u32);

impl MyStruct {
    fn set_empty(&mut self) {
        self.0 |= 0b00000_1_00;
    }
}

#[test]
fn test_set_empty_initial_state() {
    let mut my_struct = MyStruct::default();
    my_struct.set_empty();
    assert_eq!(my_struct.0, 0b00000_1_00);
}

#[test]
fn test_set_empty_already_set() {
    let mut my_struct = MyStruct(0b00000_1_00);
    my_struct.set_empty();
    assert_eq!(my_struct.0, 0b00000_1_00);
}

#[test]
fn test_set_empty_full_state() {
    let mut my_struct = MyStruct(u32::MAX);
    my_struct.set_empty();
    assert_eq!(my_struct.0, u32::MAX | 0b00000_1_00);
}

#[test]
fn test_set_empty_zero_state() {
    let mut my_struct = MyStruct(0);
    my_struct.set_empty();
    assert_eq!(my_struct.0, 0b00000_1_00);
}

#[test]
#[should_panic]
fn test_set_empty_panic_condition() {
    let mut my_struct = MyStruct(u32::MAX);
    my_struct.0 = 0; // setting to an edge case, not inherently panic but testing the reset
    my_struct.set_empty();
    assert_eq!(my_struct.0, 0b00000_1_00);
}

