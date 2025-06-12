// Answer 0

#[derive(Default)]
struct MyStruct(u32);

impl MyStruct {
    fn set_empty(&mut self) {
        self.0 |= 0b00000_1_00;
    }
}

#[test]
fn test_set_empty() {
    let mut my_struct = MyStruct::default();
    my_struct.set_empty();
    assert_eq!(my_struct.0, 0b00000_1_00);
}

#[test]
fn test_set_empty_twice() {
    let mut my_struct = MyStruct::default();
    my_struct.set_empty();
    my_struct.set_empty();
    assert_eq!(my_struct.0, 0b00000_1_00);
}

#[test]
fn test_set_empty_initially_zero() {
    let my_struct = MyStruct::default();
    assert_eq!(my_struct.0, 0);
}

#[test]
fn test_set_empty_boundary() {
    let mut my_struct = MyStruct(0b11111_0_11);
    my_struct.set_empty();
    assert_eq!(my_struct.0, 0b11111_1_11);
}

