// Answer 0

#[derive(Debug)]
struct TestStruct {
    start: char,
}

impl TestStruct {
    fn set_lower(&mut self, bound: char) {
        self.start = bound;
    }
}

#[test]
fn test_set_lower_normal_case() {
    let mut test_instance = TestStruct { start: 'a' };
    test_instance.set_lower('b');
    assert_eq!(test_instance.start, 'b');
}

#[test]
fn test_set_lower_boundary_case_lower() {
    let mut test_instance = TestStruct { start: 'a' };
    test_instance.set_lower('a');
    assert_eq!(test_instance.start, 'a');
}

#[test]
fn test_set_lower_boundary_case_upper() {
    let mut test_instance = TestStruct { start: 'z' };
    test_instance.set_lower('y');
    assert_eq!(test_instance.start, 'y');
}

#[test]
fn test_set_lower_special_character() {
    let mut test_instance = TestStruct { start: 'x' };
    test_instance.set_lower('$');
    assert_eq!(test_instance.start, '$');
}

