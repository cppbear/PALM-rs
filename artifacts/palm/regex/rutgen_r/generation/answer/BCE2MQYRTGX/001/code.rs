// Answer 0

#[derive(Default)]
struct TestStruct(u32);

impl TestStruct {
    fn set_word(&mut self) {
        self.0 |= 0b000000_1_0;
    }
}

#[test]
fn test_set_word_initial_state() {
    let mut test_instance = TestStruct::default();
    assert_eq!(test_instance.0, 0);
    test_instance.set_word();
    assert_eq!(test_instance.0, 0b000000_1_0);
}

#[test]
fn test_set_word_already_set() {
    let mut test_instance = TestStruct(0b000000_1_0);
    assert_eq!(test_instance.0, 0b000000_1_0);
    test_instance.set_word();
    assert_eq!(test_instance.0, 0b000000_1_0); // should remain unchanged
}

#[test]
fn test_set_word_without_initialization() {
    let mut test_instance = TestStruct(0);
    test_instance.set_word();
    assert_eq!(test_instance.0, 0b000000_1_0);
}

