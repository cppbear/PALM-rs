// Answer 0

#[derive(Debug)]
struct TestStruct {
    end: char,
}

impl TestStruct {
    fn upper(&self) -> char {
        self.end
    }
}

#[test]
fn test_upper() {
    let test_instance = TestStruct { end: 'Z' };
    assert_eq!(test_instance.upper(), 'Z');
}

#[test]
fn test_upper_boundary() {
    let test_instance = TestStruct { end: 'A' };
    assert_eq!(test_instance.upper(), 'A');
}

#[test]
fn test_upper_lower_case() {
    let test_instance = TestStruct { end: 'z' };
    assert_eq!(test_instance.upper(), 'z');
}

