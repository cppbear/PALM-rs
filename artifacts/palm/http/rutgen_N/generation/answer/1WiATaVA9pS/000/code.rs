// Answer 0

#[derive(Debug, PartialEq)]
enum Danger {
    Green,
    Yellow,
}

struct TestStruct(Danger);

impl TestStruct {
    fn set_yellow(&mut self) {
        if let Danger::Green = self.0 {
            self.0 = Danger::Yellow;
        }
    }
}

#[test]
fn test_set_yellow_transition_from_green() {
    let mut test_struct = TestStruct(Danger::Green);
    test_struct.set_yellow();
    assert_eq!(test_struct.0, Danger::Yellow);
}

#[test]
fn test_set_yellow_no_transition_from_yellow() {
    let mut test_struct = TestStruct(Danger::Yellow);
    test_struct.set_yellow();
    assert_eq!(test_struct.0, Danger::Yellow);
}

