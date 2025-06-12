// Answer 0

#[derive(Debug, PartialEq)]
enum Danger {
    Yellow,
    Green,
}

struct TestStruct {
    danger: Danger,
}

impl TestStruct {
    fn is_yellow(&self) -> bool {
        self.danger == Danger::Yellow
    }

    fn set_green(&mut self) {
        debug_assert!(self.is_yellow());
        *self = TestStruct { danger: Danger::Green };
    }
}

#[test]
fn test_set_green_success() {
    let mut test_struct = TestStruct { danger: Danger::Yellow };
    test_struct.set_green();
    assert_eq!(test_struct.danger, Danger::Green);
}

#[should_panic(expected = "assertion failed")]
#[test]
fn test_set_green_failure() {
    let mut test_struct = TestStruct { danger: Danger::Green };
    test_struct.set_green();
}

