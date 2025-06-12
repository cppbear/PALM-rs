// Answer 0


struct TestStruct {
    start: u8,
}

impl TestStruct {
    fn set_lower(&mut self, bound: u8) {
        self.start = bound;
    }
}

#[test]
fn test_set_lower_with_minimum_value() {
    let mut test_struct = TestStruct { start: 5 };
    test_struct.set_lower(0);
    assert_eq!(test_struct.start, 0);
}

#[test]
fn test_set_lower_with_mid_range_value() {
    let mut test_struct = TestStruct { start: 5 };
    test_struct.set_lower(127);
    assert_eq!(test_struct.start, 127);
}

#[test]
fn test_set_lower_with_maximum_value() {
    let mut test_struct = TestStruct { start: 5 };
    test_struct.set_lower(255);
    assert_eq!(test_struct.start, 255);
}

#[test]
fn test_set_lower_with_same_value() {
    let mut test_struct = TestStruct { start: 5 };
    test_struct.set_lower(5);
    assert_eq!(test_struct.start, 5);
}


