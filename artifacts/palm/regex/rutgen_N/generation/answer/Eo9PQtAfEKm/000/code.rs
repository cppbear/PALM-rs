// Answer 0

#[derive(Debug)]
struct MyStruct(u8);

impl MyStruct {
    fn is_word(&self) -> bool {
        self.0 & 0b000000_1_0 > 0
    }
}

#[test]
fn test_is_word_true() {
    let my_struct = MyStruct(0b000000_1_0);
    assert!(my_struct.is_word());
}

#[test]
fn test_is_word_false() {
    let my_struct = MyStruct(0b000000_0_0);
    assert!(!my_struct.is_word());
}

#[test]
fn test_is_word_boundary() {
    let my_struct = MyStruct(0b000000_1_1);
    assert!(my_struct.is_word());
}

