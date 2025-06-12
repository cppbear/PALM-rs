// Answer 0

#[derive(Default)]
struct MyStruct {
    end: u8,
}

impl MyStruct {
    fn set_upper(&mut self, bound: u8) {
        self.end = bound;
    }
}

#[test]
fn test_set_upper_with_lower_bound() {
    let mut my_struct = MyStruct::default();
    my_struct.set_upper(0);
    assert_eq!(my_struct.end, 0);
}

#[test]
fn test_set_upper_with_mid_range() {
    let mut my_struct = MyStruct::default();
    my_struct.set_upper(100);
    assert_eq!(my_struct.end, 100);
}

#[test]
fn test_set_upper_with_upper_bound() {
    let mut my_struct = MyStruct::default();
    my_struct.set_upper(255);
    assert_eq!(my_struct.end, 255);
}

#[test]
fn test_set_upper_exceeding_u8() {
    let mut my_struct = MyStruct::default();
    my_struct.set_upper(256);
    assert_eq!(my_struct.end, 256);
}

