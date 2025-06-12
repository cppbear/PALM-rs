// Answer 0

#[derive(Debug)]
struct TestStruct {
    start: char,
}

impl TestStruct {
    fn lower(&self) -> char {
        self.start
    }
}

#[test]
fn test_lower_with_valid_char() {
    let test_instance = TestStruct { start: 'a' };
    assert_eq!(test_instance.lower(), 'a');
}

#[test]
fn test_lower_with_uppercase_char() {
    let test_instance = TestStruct { start: 'Z' };
    assert_eq!(test_instance.lower(), 'Z');
}

#[test]
fn test_lower_with_numeric_char() {
    let test_instance = TestStruct { start: '1' };
    assert_eq!(test_instance.lower(), '1');
}

#[test]
fn test_lower_with_special_char() {
    let test_instance = TestStruct { start: '!' };
    assert_eq!(test_instance.lower(), '!');
}

#[test]
fn test_lower_with_space_char() {
    let test_instance = TestStruct { start: ' ' };
    assert_eq!(test_instance.lower(), ' ');
}

